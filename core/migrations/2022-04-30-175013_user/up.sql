ALTER TABLE IF EXISTS users ADD COLUMN IF NOT EXISTS pepper text;
CREATE TABLE if not exists users
(
 id        text,
 group_id      text NOT NULL,
 username      text NOT NULL,
 email         text NOT NULL,
 password      text NOT NULL,
 pepper        text NOT NULL,
 creation_time timestamp NOT NULL,
 CONSTRAINT PK_users PRIMARY KEY (id)
);


