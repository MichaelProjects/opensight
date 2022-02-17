create table if not exists sessions (
    id text primary key,
    application_id text not null,
    first_today boolean not null,
    length integer not null,
    start_time timestamp not null
);