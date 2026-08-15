-- Your SQL goes here
create table cards (
    id uuid primary key,
    state integer not null,
    due integer not null,
    last_review integer not null,
    stability real not null,
    difficulty real not null
);
