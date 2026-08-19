-- Your SQL goes here
create table cards (
    id blob primary key not null,
    state integer not null,
    due bigint not null,
    last_review bigint not null,
    stability real not null,
    difficulty real not null
);
