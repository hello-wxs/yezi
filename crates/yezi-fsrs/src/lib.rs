// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, diesel::AsExpression, diesel::FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Binary)]
pub struct Ulid(pub ulid::Ulid);

impl Ulid {
    pub fn generate() -> Self {
        Self(ulid::Ulid::generate())
    }
}

impl diesel::deserialize::FromSql<diesel::sql_types::Binary, diesel::sqlite::Sqlite> for Ulid {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let blob = <Vec<u8> as diesel::deserialize::FromSql<
            diesel::sql_types::Binary,
            diesel::sqlite::Sqlite,
        >>::from_sql(bytes)?;
        let arr: [u8; 16] = blob
            .try_into()
            .map_err(|e: Vec<u8>| anyhow::anyhow!("Expected 16 bytes, got {}", e.len()))?;
        Ok(Ulid(ulid::Ulid::from_bytes(arr)))
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Binary, diesel::sqlite::Sqlite> for Ulid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        let bytes = self.0.to_bytes();
        out.set_value(bytes.to_vec());
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(
    Debug,
    PartialEq,
    Clone,
    Copy,
    diesel::Queryable,
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
)]
#[diesel(table_name = yezi_db::schema::cards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Card {
    // Metadata
    pub id: Ulid,
    pub state: CardState,
    // Time state
    pub due: i64,
    pub last_review: i64,
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
    pub fn new(id: Ulid) -> Self {
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
        parameters: &fsrs::FSRS,
        rating: Rating,
        desired_retention: f32,
        kind: ReviewKind,
        taken_time: u64,
    ) -> Result<ReviewLog> {
        let system_time = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_secs();
        let spaced_time = system_time - self.last_review as u64;
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
        self.last_review = system_time as i64;
        self.stability = chosen_state.memory.stability;
        self.difficulty = chosen_state.memory.difficulty;
        self.due = (chosen_state.interval * 86400.0) as i64 + system_time as i64;
        self.state.transition(rating);
        Ok(ReviewLog::new(
            self.id,
            kind,
            system_time as i64,
            spaced_time as i64,
            rating,
            taken_time as i64,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, diesel::FromSqlRow, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Integer)]
pub enum CardState {
    New = 0,
    Learning = 1,
    Review = 2,
    Relearning = 3,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite>
    for CardState
{
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let num = i32::from_sql(bytes)?;
        match num {
            0 => Ok(CardState::New),
            1 => Ok(CardState::Learning),
            2 => Ok(CardState::Review),
            3 => Ok(CardState::Relearning),
            _ => Err(anyhow::anyhow!("Invalid CardState value {num}"))?,
        }
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite> for CardState {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        match self {
            CardState::New => out.set_value(0),
            CardState::Learning => out.set_value(1),
            CardState::Review => out.set_value(2),
            CardState::Relearning => out.set_value(3),
        }
        Ok(diesel::serialize::IsNull::No)
    }
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

#[derive(
    Debug, PartialEq, Clone, Copy, diesel::Queryable, diesel::Selectable, diesel::Insertable,
)]
#[diesel(table_name = yezi_db::schema::reviews)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ReviewLog {
    // Metadata
    pub id: Ulid,
    pub card_id: Ulid,
    pub kind: ReviewKind,
    // Time state
    pub reviewed_at: i64,
    pub spaced_time: i64,
    // User feedback
    pub rating: Rating,
    pub taken_time: i64,
}

impl ReviewLog {
    pub fn new(
        card_id: Ulid,
        kind: ReviewKind,
        reviewed_at: i64,
        spaced_time: i64,
        rating: Rating,
        taken_time: i64,
    ) -> Self {
        Self {
            id: Ulid::generate(),
            card_id,
            kind,
            reviewed_at,
            spaced_time,
            rating,
            taken_time,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, diesel::FromSqlRow, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Integer)]
pub enum Rating {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite> for Rating {
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let num = i32::from_sql(bytes)?;
        match num {
            1 => Ok(Self::Again),
            2 => Ok(Self::Hard),
            3 => Ok(Self::Good),
            4 => Ok(Self::Easy),
            _ => Err(anyhow::anyhow!("invalid rating: {num}"))?,
        }
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite> for Rating {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        match self {
            Self::Again => out.set_value(1),
            Self::Hard => out.set_value(2),
            Self::Good => out.set_value(3),
            Self::Easy => out.set_value(4),
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, diesel::FromSqlRow, diesel::AsExpression)]
#[diesel(sql_type = diesel::sql_types::Integer)]
pub enum ReviewKind {
    Auto = 0,
    Manual = 1,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite>
    for ReviewKind
{
    fn from_sql(
        bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let num = i32::from_sql(bytes)?;
        match num {
            0 => Ok(ReviewKind::Auto),
            1 => Ok(ReviewKind::Manual),
            _ => Err(anyhow::anyhow!("invalid review kind: {num}"))?,
        }
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite> for ReviewKind {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        match self {
            Self::Auto => out.set_value(0),
            Self::Manual => out.set_value(1),
        }
        Ok(diesel::serialize::IsNull::No)
    }
}

pub struct Learn {
    pub parameters: fsrs::FSRS,
    pub cards: std::collections::BinaryHeap<Card>,
    pub connection: diesel::SqliteConnection,
}

impl std::fmt::Debug for Learn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Learn")
            .field("parameters", &self.parameters)
            .field("cards", &self.cards)
            .finish()
    }
}

impl Learn {
    pub fn new(parameters: fsrs::FSRS, db_path: &std::path::Path) -> Result<Self> {
        use diesel::RunQueryDsl;
        use yezi_db::schema::cards::dsl::*;

        let mut connection = yezi_db::init_database(db_path)?;
        Ok(Self {
            parameters,
            cards: std::collections::BinaryHeap::from(cards.load::<Card>(&mut connection)?),
            connection,
        })
    }
    pub fn next_time(&self) -> Option<i64> {
        self.cards.peek().map(|card| card.due)
    }
    pub fn next_id(&self) -> Option<Ulid> {
        self.cards.peek().map(|card| card.id)
    }
    pub fn is_dued(&self) -> Result<bool> {
        if let Some(card) = self.cards.peek() {
            let system_time = std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)?
                .as_secs() as i64;
            Ok(card.due <= system_time)
        } else {
            Ok(false)
        }
    }
    pub fn learn(
        &mut self,
        rating: Rating,
        desired_retention: f32,
        kind: ReviewKind,
        taken_time: u64,
    ) -> Result<()> {
        use diesel::{QueryDsl, RunQueryDsl};

        let mut card = self.cards.pop().ok_or(Error::NoDuedCard)?;
        let log = card.learn(
            &self.parameters,
            rating,
            desired_retention,
            kind,
            taken_time,
        )?;
        self.cards.push(card);
        diesel::update(yezi_db::schema::cards::dsl::cards.find(card.id))
            .set(card)
            .execute(&mut self.connection)?;
        diesel::insert_into(yezi_db::schema::reviews::dsl::reviews)
            .values(log)
            .execute(&mut self.connection)?;
        Ok(())
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("system time may have gone backwards")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("fsrs error")]
    FSRSError(#[from] fsrs::FSRSError),
    #[error("no dued card")]
    NoDuedCard,
    #[error("db connect or migrate failed")]
    DBConnectMigrateFailed(#[from] yezi_db::Error),
    #[error("db error")]
    DBError(#[from] diesel::result::Error),
}
