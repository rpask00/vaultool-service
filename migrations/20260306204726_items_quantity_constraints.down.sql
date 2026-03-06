-- Add down migration script here
ALTER TABLE items
    DROP CONSTRAINT IF EXISTS items_quantity_non_negative;

ALTER TABLE items
    ALTER COLUMN quantity DROP NOT NULL;