CREATE TABLE if not exists projects (
    id text NOT NULL,
    projects_name text NOT NULL,
    created timestamp NOT NULL,
    updated timestamp NOT NULL,
CONSTRAINT PK_projects PRIMARY KEY (id)
);