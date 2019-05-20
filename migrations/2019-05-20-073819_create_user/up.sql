-- Your SQL goes here

CREATE TABLE Doctor(
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  birthdate DATE NOT NULL
);