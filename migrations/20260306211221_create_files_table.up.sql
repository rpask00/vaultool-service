-- Add up migration script here
CREATE table if not exists files
(
    id         SERIAL PRIMARY KEY,
    item_id    INT REFERENCES items (id) ON DELETE CASCADE NOT NULL,
    name       TEXT,
    category   INT NOT NULL,
    created_at TIMESTAMP,
    size       INT
)