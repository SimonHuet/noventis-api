#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::doctors;
use doctors::Doctor;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Doctor>> {
    doctors::table.load::<Doctor>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Doctor> {
    doctors::table.find(id).get_result::<Doctor>(connection)
}

pub fn insert(doctor: InsertableDoctor, connection: &PgConnection) -> QueryResult<Doctor> {
    diesel::insert_into(doctors::table)
        .values(doctor)
        .get_result(connection)
}

pub fn update(id: i32, doctor: InsertableDoctor, connection: &PgConnection) -> QueryResult<Doctor> {
    diesel::update(doctors::table.find(id))
        .set(&doctor)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(doctors::table.find(id))
        .execute(connection)
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize )]
#[table_name = "doctors"]
pub struct InsertableDoctor {
    first_name: String,
    last_name: String,
    birthdate: String
}