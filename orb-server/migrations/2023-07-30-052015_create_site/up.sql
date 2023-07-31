-- SQLite 3
-- Your SQL goes here
CREATE TABLE sites (
    id BLOB PRIMARY KEY NOT NULL UNIQUE,
    module_id BLOB NOT NULL UNIQUE,
    name TEXT NOT NULL
)
