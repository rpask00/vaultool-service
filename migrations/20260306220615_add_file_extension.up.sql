-- Add up migration script here
ALTER TABLE files
    ADD COLUMN extension TEXT NOT NULL DEFAULT 'jpg';