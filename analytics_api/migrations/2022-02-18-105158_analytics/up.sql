ALTER TABLE if exists analytics drop column if exists "last_session";
CREATE TABLE if not exists analytics ( 
    session_id text NOT NULL, 
    application_id text NOT NULL, 
    creation_time timestamp NOT NULL, 
    os text NOT NULL, 
    device_size text NOT NULL, 
    new_user boolean NOT NULL, 
    country text NOT NULL, 
    device_type text NOT NULL, 
    version text NOT NULL, 
    CONSTRAINT PK_analytics PRIMARY KEY ( session_id, application_id ) );