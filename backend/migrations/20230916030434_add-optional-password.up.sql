-- Add up migration script here
ALTER TABLE user ADD uses_password BOOLEAN NOT NULL DEFAULT FALSE;
