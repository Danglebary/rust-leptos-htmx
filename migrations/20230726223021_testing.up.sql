-- Add up migration script here
CREATE TABLE IF NOT EXISTS todos (
    id integer PRIMARY KEY NOT NULL,
    description varchar(255) NOT NULL,
    done boolean NOT NULL DEFAULT false
)