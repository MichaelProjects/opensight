create table if not exists sessions (
    id text primary key,
    application_id text not null,
    length int not null,
    start_time timestamp not null
);