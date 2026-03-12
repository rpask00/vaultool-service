-- Add up migration script here

ALTER TABLE files
    ALTER COLUMN item_id DROP NOT NULL;