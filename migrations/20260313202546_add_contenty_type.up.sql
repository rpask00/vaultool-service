-- Add up migration script here

ALTER TABLE files
    ADD COLUMN content_type VARCHAR(255) NOT NULL DEFAULT 'image/jpeg';