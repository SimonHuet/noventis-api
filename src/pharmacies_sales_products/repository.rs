#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::pharmacies_sales_products;
use pharmacies_sales_products::PharmacySaleProduct;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<PharmacySaleProduct>> {
    pharmacies_sales_products::table.load::<PharmacySaleProduct>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<PharmacySaleProduct> {
    pharmacies_sales_products::table.find(id).get_result::<PharmacySaleProduct>(connection)
}

pub fn insert(pharmacy_purchase_product: InsertablePharmacySaleProduct, connection: &PgConnection) -> QueryResult<PharmacySaleProduct> {
    diesel::insert_into(pharmacies_sales_products::table)
        .values(pharmacy_purchase_product)
        .get_result(connection)
}

pub fn update(id: i32, pharmacy_purchase_product: InsertablePharmacySaleProduct, connection: &PgConnection) -> QueryResult<PharmacySaleProduct> {
    diesel::update(pharmacies_sales_products::table.find(id))
        .set(&pharmacy_purchase_product)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(pharmacies_sales_products::table.find(id))
        .execute(connection)
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize )]
#[table_name = "pharmacies_sales_products"]
pub struct InsertablePharmacySaleProduct {
    pub quantity: i32,
    pub date: String,
    pub price: i32,
    pub pharmacies_id: i32,
    pub products_id: i32
}