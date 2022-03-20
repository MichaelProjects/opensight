alter table if exists sessions ADD COLUMN is_first_login_today boolean;

create table if not exists sessions (
    id text primary key,
    application_id text not null,
    length int not null,
    is_first_login_today boolean not null,
    start_time timestamp not null
);

