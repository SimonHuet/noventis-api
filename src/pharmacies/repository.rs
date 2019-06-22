#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::pharmacies;
use pharmacies::Pharmacy;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Pharmacy>> {
    pharmacies::table.load::<Pharmacy>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Pharmacy> {
    pharmacies::table.find(id).get_result::<Pharmacy>(connection)
}

pub fn insert(pharmacy: InsertablePharmacy, connection: &PgConnection) -> QueryResult<Pharmacy> {
    diesel::insert_into(pharmacies::table)
        .values(pharmacy)
        .get_result(connection)
}

pub fn update(id: i32, pharmacy: InsertablePharmacy, connection: &PgConnection) -> QueryResult<Pharmacy> {
    diesel::update(pharmacies::table.find(id))
        .set(&pharmacy)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(pharmacies::table.find(id))
        .execute(connection)
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize )]
#[table_name = "pharmacies"]
pub struct InsertablePharmacy {
    pub street: String,
    pub street_number: i32,
    pub city : String,
    pub longitude : f64,
    pub latitude : f64    
}