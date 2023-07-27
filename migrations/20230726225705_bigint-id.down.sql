-- Add down migration script here
ALTER TABLE todos ALTER COLUMN id TYPE integer NOT NULL;