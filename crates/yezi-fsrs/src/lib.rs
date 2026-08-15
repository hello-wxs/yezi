// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, PartialEq)]
pub struct Card {
    // Metadata
    pub id: uuid::Uuid,
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
    pub fn new(id: uuid::Uuid) -> Self {
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
    ) -> Result<()> {
        let system_time = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        let days_elapsed = ((system_time - self.last_review) / 86400) as u32;
        let memory_state = match self.state {
            CardState::New => None,
            _ => Some(fsrs::MemoryState {
                stability: self.stability,
                difficulty: self.difficulty,
            }),
        };
        let next_states = parameters.next_states(memory_state, desired_retention, days_elapsed)?;
        match rating {
            Rating::Again => {
                self.stability = next_states.again.memory.stability;
                self.difficulty = next_states.again.memory.difficulty;
                self.due = (next_states.again.interval * 86400.0) as u64 + system_time;
            }
            Rating::Hard => {
                self.stability = next_states.hard.memory.stability;
                self.difficulty = next_states.hard.memory.difficulty;
                self.due = (next_states.hard.interval * 86400.0) as u64 + system_time;
            }
            Rating::Good => {
                self.stability = next_states.good.memory.stability;
                self.difficulty = next_states.good.memory.difficulty;
                self.due = (next_states.good.interval * 86400.0) as u64 + system_time;
            }
            Rating::Easy => {
                self.stability = next_states.easy.memory.stability;
                self.difficulty = next_states.easy.memory.difficulty;
                self.due = (next_states.easy.interval * 86400.0) as u64 + system_time;
            }
        }
        self.state.transition(rating);
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct ReviewLog {
    // Metadata
    pub id: uuid::Uuid,
    pub card_id: uuid::Uuid,
    pub kind: ReviewKind,
    // Time state
    pub reviewed_at: u64,
    pub spaced_time: u64,
    // User feedback
    pub rating: Rating,
    pub taken_time: u64,
}

#[derive(Debug)]
pub enum Rating {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}

#[derive(Debug)]
pub enum ReviewKind {
    Learning = 0,
    Review = 1,
    Relearning = 2,
    Filtered = 3,
    Manual = 4,
}

#[derive(Debug)]
pub struct Learn {
    pub parameters: fsrs::FSRS,
    pub cards: std::collections::BinaryHeap<Card>,
}

impl Learn {
    pub fn current_learn(&self) -> Option<uuid::Uuid> {
        self.cards.peek().map(|c| c.id)
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("system time may have gone backwards")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("fsrs error")]
    FSRSSrror(#[from] fsrs::FSRSError),
}
