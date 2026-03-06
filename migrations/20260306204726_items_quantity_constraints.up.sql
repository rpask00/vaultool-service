-- Add up migration script here
ALTER TABLE items
    ALTER COLUMN quantity SET NOT NULL;

ALTER TABLE items
    ADD CONSTRAINT items_quantity_non_negative
        CHECK (quantity >= 0);