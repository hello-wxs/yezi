-- Your SQL goes here
create table cards (
    id blob primary key not null,
    state integer not null,
    due integer not null,
    last_review integer not null,
    stability real not null,
    difficulty real not null
);
