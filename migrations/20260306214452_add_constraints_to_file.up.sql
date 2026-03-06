-- Add up migration script here
ALTER TABLE files
    ALTER COLUMN name SET NOT NULL;

ALTER TABLE files
    ALTER COLUMN created_at SET DEFAULT CURRENT_TIMESTAMP;

ALTER TABLE files
    ALTER COLUMN created_at SET NOT NULL;

ALTER TABLE files
    ALTER COLUMN size SET NOT NULL;

ALTER TABLE files
    ADD CONSTRAINT files_size_non_negative
        CHECK (size >= 0);