#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::formations;
use formations::Formation;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Formation>> {
    formations::table.load::<Formation>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Formation> {
    formations::table.find(id).get_result::<Formation>(connection)
}

pub fn insert(formation: InsertableFormation, connection: &PgConnection) -> QueryResult<Formation> {
    diesel::insert_into(formations::table)
        .values(formation)
        .get_result(connection)
}

pub fn update(id: i32, formation: InsertableFormation, connection: &PgConnection) -> QueryResult<Formation> {
    diesel::update(formations::table.find(id))
        .set(formation)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(formations::table.find(id))
        .execute(connection)
}

#[derive(Insertable, AsChangeset , Serialize, Deserialize)]
#[table_name = "formations"]
pub struct InsertableFormation {
    name: String,
    description: String,
}