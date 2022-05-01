CREATE TABLE if not exists users
(
 id        text NOT NULL,
 group_id      text NOT NULL,
 username      text NOT NULL,
 email         text NOT NULL,
 password      text NOT NULL,
 creation_time timestamp NOT NULL,
CONSTRAINT PK_users PRIMARY KEY (id)
);


