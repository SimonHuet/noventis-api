#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::formations;

pub mod handler;
pub mod repository;
pub mod router;


#[derive(Queryable, AsChangeset ,Serialize, Deserialize)]
pub struct Formation {
    pub id: i32,
    pub name: String,
    pub description: String
}