#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::doctors_formations;
use doctors_formations::DoctorFormation;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<DoctorFormation>> {
    doctors_formations::table.load::<DoctorFormation>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<DoctorFormation> {
    doctors_formations::table.find(id).get_result::<DoctorFormation>(connection)
}

pub fn insert(doctorformation: InsertableDoctorFormation, connection: &PgConnection) -> QueryResult<DoctorFormation> {
    diesel::insert_into(doctors_formations::table)
        .values(doctorformation)
        .get_result(connection)
}

pub fn update(id: i32, doctorformation: InsertableDoctorFormation, connection: &PgConnection) -> QueryResult<DoctorFormation> {
    diesel::update(doctors_formations::table.find(id))
        .set(&doctorformation)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(doctors_formations::table.find(id))
        .execute(connection)
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize )]
#[table_name = "doctors_formations"]
pub struct InsertableDoctorFormation {
    doctor_id: i32,
    formation_id: i32,
}