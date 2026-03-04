-- Add up migration script here
CREATE TABLE IF NOT EXISTS items
(
    id          SERIAL PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT NOT NULL,
    quantity    INTEGER NOT NULL,
    tags        TEXT[] NOT NULL
);