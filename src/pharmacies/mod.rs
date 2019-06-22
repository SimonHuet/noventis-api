#![allow(proc_macro_derive_resolution_fallback)]
use schema::pharmacies;

pub mod handler;
pub mod repository;
pub mod router;
#[cfg(test)]
pub mod test;

#[derive(Queryable,Identifiable,AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "pharmacies"]
pub struct Pharmacy {
    pub id: i32,
    pub street: String,
    pub street_number: i32,
    pub city: String,
    pub longitude: f64,
    pub latitude: f64 
}