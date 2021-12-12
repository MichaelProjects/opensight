CREATE TABLE if not exists Applications
(
 application_id   text NOT NULL,
 application_name text NOT NULL,
 creation_time    timestamp NOT NULL,
 token            text NOT NULL,
 os               text NOT NULL,
 CONSTRAINT PK_applications PRIMARY KEY ( application_id )
);


