-- Your SQL goes here

CREATE TABLE pharmacies(
    id SERIAL PRIMARY KEY,
    street VARCHAR NOT NULL,
    street_number INTEGER NOT NULL,
    city VARCHAR NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    latitude  DOUBLE PRECISION NOT NULL
)