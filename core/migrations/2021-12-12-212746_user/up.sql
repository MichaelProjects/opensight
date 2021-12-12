CREATE TABLE if not exists opensight_user
(
 userid        text NOT NULL,
 group_id      text NOT NULL,
 username      text NOT NULL,
 email         text NOT NULL,
 password      text NOT NULL,
 creation_time timestamp NOT NULL,
 CONSTRAINT PK_user PRIMARY KEY ( userid, group_id ),
 CONSTRAINT FK_178 FOREIGN KEY ( group_id ) REFERENCES user_group ( group_id )
);

CREATE INDEX FK_180 ON opensight_user
(
 group_id
);



