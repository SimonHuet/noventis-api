#![allow(proc_macro_derive_resolution_fallback)]
use schema::pharmacies_purchases_products;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Queryable,Identifiable,AsChangeset, Serialize, Deserialize, Associations)]
#[table_name = "pharmacies_purchases_products"]
pub struct PharmacyPurchaseProduct {
    pub quantity: i32,
    pub date: String,
    pub price: i32,
    pub pharmacies_id: i32,
    pub products_id: i32,
    pub id: i32

}