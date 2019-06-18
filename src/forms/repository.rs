#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::forms;
use forms::Form;
use serde_json::Value;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Form>> {
    forms::table.load::<Form>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Form> {
    forms::table.find(id).get_result::<Form>(connection)
}

pub fn insert(form: InsertableForm, connection: &PgConnection) -> QueryResult<Form> {
    diesel::insert_into(forms::table)
        .values(form)
        .get_result(connection)
}

pub fn update(id: i32, form: InsertableForm, connection: &PgConnection) -> QueryResult<Form> {
    diesel::update(forms::table.find(id))
        .set(&form)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(forms::table.find(id))
        .execute(connection)
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize )]
#[table_name = "forms"]
pub struct InsertableForm {
    pub created_at: String,
    pub form: Value
}