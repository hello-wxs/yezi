-- Your SQL goes here
create table reviews (
    id blob primary key not null,
    card_id blob not null,
    kind integer not null,
    review_at bigint not null,
    spaced_time bigint not null,
    rating integer not null,
    taken_time bigint not null
);
