#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::products;
use products::Product;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Product>> {
    products::table.load::<Product>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Product> {
    products::table.find(id).get_result::<Product>(connection)
}

pub fn insert(product: InsertableProduct, connection: &PgConnection) -> QueryResult<Product> {
    diesel::insert_into(products::table)
        .values(product)
        .get_result(connection)
}

pub fn update(id: i32, product: InsertableProduct, connection: &PgConnection) -> QueryResult<Product> {
    diesel::update(products::table.find(id))
        .set(&product)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(products::table.find(id))
        .execute(connection)
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize )]
#[table_name = "products"]
pub struct InsertableProduct {
    pub name: String
}