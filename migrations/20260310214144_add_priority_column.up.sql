-- Add up migration script here
ALTER TABLE files
    ADD COLUMN priority INTEGER NOT NULL DEFAULT 0

