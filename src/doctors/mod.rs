#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::doctors;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Queryable,Identifiable, AsChangeset, Serialize, Deserialize)]
pub struct Doctor {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: String
}