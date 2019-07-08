#![allow(proc_macro_derive_resolution_fallback)]
use schema::pharmacies_sales_products;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Queryable,Identifiable,AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "pharmacies_sales_products"]
pub struct PharmacySaleProduct {
    pub quantity: i32,
    pub date: String,
    pub price: i32,
    pub pharmacies_id: i32,
    pub products_id: i32,
    pub id: i32

}