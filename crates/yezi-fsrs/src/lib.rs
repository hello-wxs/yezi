// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

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

pub enum Rating {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}

pub enum ReviewKind {
    Learning = 0,
    Review = 1,
    Relearning = 2,
    Filtered = 3,
    Manual = 4,
}
