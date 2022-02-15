CREATE TABLE if not exists users
(
 userid        text primary key,
 group_id      text NOT NULL,
 username      text NOT NULL,
 email         text NOT NULL,
 password      text NOT NULL,
 creation_time timestamp NOT NULL
);


