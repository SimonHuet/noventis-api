-- Your SQL goes here

CREATE TABLE Doctors(
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  birthdate VARCHAR NOT NULL
);