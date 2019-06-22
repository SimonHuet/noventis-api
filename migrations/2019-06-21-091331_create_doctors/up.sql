-- Your SQL goes here

CREATE TABLE doctors(
  id SERIAL PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  birthdate VARCHAR NOT NULL,
  pharmacy_id INT REFERENCES pharmacies(id)
);