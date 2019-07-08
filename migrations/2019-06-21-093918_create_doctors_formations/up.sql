-- Your SQL goes here


CREATE TABLE doctors_formations(
    id SERIAL PRIMARY KEY,
    doctor_id SERIAL references doctors(id),
    formation_id SERIAL references formations(id)
    
);