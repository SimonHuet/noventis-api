#![allow(proc_macro_derive_resolution_fallback)]
use chrono::NaiveDateTime;
use super::super::schema::doctors;

use diesel;
use diesel::QueryResult;
use diesel::prelude::*;


#[derive(Queryable, Serialize, Deserialize)]
pub struct Doctor {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDateTime,
}

#[derive(Debug, Queryable ,Insertable, Serialize)]
#[table_name = "doctors"]
pub struct InsertableDoctor {
    first_name: String,
    last_name: String,
    birthdate: NaiveDateTime
}
impl InsertableDoctor {
    fn from_doctor(doctor: Doctor) -> InsertableDoctor {
        InsertableDoctor {
            first_name: doctor.first_name,
            last_name: doctor.last_name,
            birthdate: doctor.birthdate
        }
    }
}

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Doctor>> {
    doctors::table.load::<Doctor>(&*connection);
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Doctor> {
    doctors::table.find(id).get_result::<Doctor>(connection)
}

pub fn insert(doctor: Doctor, connection: &PgConnection) -> QueryResult<Doctor> {
    diesel::insert_into(doctors::table)
        .values(&InsertableDoctor::from_doctor(doctor))
        .get_result(connection)
}

pub fn update(id: i32, doctor: Doctor, connection: &PgConnection) -> QueryResult<Doctor> {
    diesel::update(doctors::table.find(id)).set(&doctor)
    .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(doctors::table.find(id))
        .execute(connection)
}
