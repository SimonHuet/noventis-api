-- Your SQL goes here

CREATE TABLE forms(
    id SERIAL PRIMARY KEY,
    created_at VARCHAR NOT NULL,
    form JSON NOT NULL
)