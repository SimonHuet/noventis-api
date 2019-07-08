#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::pharmacies_purchases_products;
use pharmacies_purchases_products::PharmacyPurchaseProduct;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<PharmacyPurchaseProduct>> {
    pharmacies_purchases_products::table.load::<PharmacyPurchaseProduct>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<PharmacyPurchaseProduct> {
    pharmacies_purchases_products::table.find(id).get_result::<PharmacyPurchaseProduct>(connection)
}

pub fn insert(pharmacy_purchase_product: InsertablePharmacyPurchaseProduct, connection: &PgConnection) -> QueryResult<PharmacyPurchaseProduct> {
    diesel::insert_into(pharmacies_purchases_products::table)
        .values(pharmacy_purchase_product)
        .get_result(connection)
}

pub fn update(id: i32, pharmacy_purchase_product: InsertablePharmacyPurchaseProduct, connection: &PgConnection) -> QueryResult<PharmacyPurchaseProduct> {
    diesel::update(pharmacies_purchases_products::table.find(id))
        .set(&pharmacy_purchase_product)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(pharmacies_purchases_products::table.find(id))
        .execute(connection)
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize )]
#[table_name = "pharmacies_purchases_products"]
pub struct InsertablePharmacyPurchaseProduct {
    pub quantity: i32,
    pub date: String,
    pub price: i32,
    pub pharmacies_id: i32,
    pub products_id: i32
}