// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    // Metadata
    pub id: ulid::Ulid,
    pub state: CardState,
    // Time state
    pub due: u64,
    pub last_review: u64,
    // Memory state
    pub stability: f32,
    pub difficulty: f32,
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.due.cmp(&self.due)
    }
}

impl Card {
    pub fn new(id: ulid::Ulid) -> Self {
        Self {
            id,
            state: CardState::New,
            due: 0,
            last_review: 0,
            stability: 0.0,
            difficulty: 0.0,
        }
    }
    pub fn learn(
        &mut self,
        parameters: fsrs::FSRS,
        rating: Rating,
        desired_retention: f32,
        kind: ReviewKind,
        taken_time: u64,
    ) -> Result<ReviewLog> {
        let system_time = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        let spaced_time = system_time - self.last_review;
        let days_elapsed = (spaced_time / 86400) as u32;
        let memory_state = match self.state {
            CardState::New => None,
            _ => Some(fsrs::MemoryState {
                stability: self.stability,
                difficulty: self.difficulty,
            }),
        };
        let next_states = parameters.next_states(memory_state, desired_retention, days_elapsed)?;
        let chosen_state = match rating {
            Rating::Again => &next_states.again,
            Rating::Hard => &next_states.hard,
            Rating::Good => &next_states.good,
            Rating::Easy => &next_states.easy,
        };
        self.last_review = system_time;
        self.stability = chosen_state.memory.stability;
        self.difficulty = chosen_state.memory.difficulty;
        self.due = (chosen_state.interval * 86400.0) as u64 + system_time;
        self.state.transition(rating);
        Ok(ReviewLog::new(
            self.id,
            kind,
            system_time,
            spaced_time,
            rating,
            taken_time,
        ))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardState {
    New = 0,
    Learning = 1,
    Review = 2,
    Relearning = 3,
}

impl CardState {
    pub fn transition(&mut self, rating: Rating) {
        *self = match self {
            CardState::New => match rating {
                Rating::Again | Rating::Hard | Rating::Good => CardState::Learning,
                Rating::Easy => CardState::Review,
            },
            CardState::Learning => match rating {
                Rating::Again | Rating::Hard => CardState::Learning,
                Rating::Good | Rating::Easy => CardState::Review,
            },
            CardState::Review => match rating {
                Rating::Again => CardState::Relearning,
                Rating::Hard | Rating::Good | Rating::Easy => CardState::Review,
            },
            CardState::Relearning => match rating {
                Rating::Again | Rating::Hard => CardState::Relearning,
                Rating::Good | Rating::Easy => CardState::Review,
            },
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReviewLog {
    // Metadata
    pub id: ulid::Ulid,
    pub card_id: ulid::Ulid,
    pub kind: ReviewKind,
    // Time state
    pub reviewed_at: u64,
    pub spaced_time: u64,
    // User feedback
    pub rating: Rating,
    pub taken_time: u64,
}

impl ReviewLog {
    pub fn new(
        card_id: ulid::Ulid,
        kind: ReviewKind,
        reviewed_at: u64,
        spaced_time: u64,
        rating: Rating,
        taken_time: u64,
    ) -> Self {
        Self {
            id: ulid::Ulid::generate(),
            card_id,
            kind,
            reviewed_at,
            spaced_time,
            rating,
            taken_time,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rating {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}

#[derive(Debug, Clone, Copy)]
pub enum ReviewKind {
    Auto = 0,
    Manual = 1,
}

#[derive(Debug, Clone)]
pub struct Learn {
    pub parameters: fsrs::FSRS,
    pub cards: std::collections::BinaryHeap<Card>,
}

impl Learn {
    pub fn current_learn(&self) -> Option<ulid::Ulid> {
        self.cards.peek().map(|c| c.id)
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("system time may have gone backwards")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("fsrs error")]
    FSRSError(#[from] fsrs::FSRSError),
}
