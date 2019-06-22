#![allow(proc_macro_derive_resolution_fallback)]
use schema::doctors;

pub mod handler;
pub mod repository;
pub mod router;
#[cfg(test)]
pub mod test;

use pharmacies::Pharmacy;

#[derive(Queryable,Identifiable, AsChangeset, Serialize, Deserialize, Associations)]
#[belongs_to(Pharmacy)]
pub struct Doctor {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: String,
    pub pharmacy_id: Option<i32>
}