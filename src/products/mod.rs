#![allow(proc_macro_derive_resolution_fallback)]
use schema::products;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Queryable,Identifiable,AsChangeset, Serialize, Deserialize, Associations)]
pub struct Product {
    pub id: i32,
    pub name: String
}