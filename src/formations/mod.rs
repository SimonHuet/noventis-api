#![allow(proc_macro_derive_resolution_fallback)]
use schema::formations;

pub mod handler;
pub mod repository;
pub mod router;
pub mod test;

#[derive(Queryable, Identifiable, AsChangeset, Serialize, Deserialize, Associations)]
pub struct Formation {
    pub id: i32,
    pub name: String,
    pub description: String
}