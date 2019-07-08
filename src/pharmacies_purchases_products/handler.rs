use connection::DbConn;
use diesel::result::Error;
use std::env;
use pharmacies_purchases_products;
use pharmacies_purchases_products::PharmacyPurchaseProduct;
use rocket::http::Status;
use rocket::response::status;
use pharmacies_purchases_products::repository::InsertablePharmacyPurchaseProduct;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<PharmacyPurchaseProduct>>, Status> {
    pharmacies_purchases_products::repository::all(&connection)
        .map(|pharmacy_purchase_product| Json(pharmacy_purchase_product))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<PharmacyPurchaseProduct>, Status> {
    pharmacies_purchases_products::repository::get(id, &connection)
        .map(|pharmacy_purchase_product| Json(pharmacy_purchase_product))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<pharmacy_purchase_product>")]
pub fn post(pharmacy_purchase_product: Json<InsertablePharmacyPurchaseProduct>, connection: DbConn) -> Result<status::Created<Json<PharmacyPurchaseProduct>>, Status> {
    pharmacies_purchases_products::repository::insert(pharmacy_purchase_product.into_inner(), &connection)
        .map(|pharmacy_purchase_product| pharmacy_purchase_product_created(pharmacy_purchase_product))
        .map_err(|error| error_status(error))
}

fn pharmacy_purchase_product_created(pharmacy_purchase_product: PharmacyPurchaseProduct) -> status::Created<Json<PharmacyPurchaseProduct>> {
    
    status::Created(
        format!("{host}:{port}/pharmacyPurchaseProduct/{id}", host = host(), port = port(), id=pharmacy_purchase_product.id).to_string(),
        Some(Json(pharmacy_purchase_product)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<pharmacy_purchase_product>")]
pub fn put(id: i32, pharmacy_purchase_product: Json<InsertablePharmacyPurchaseProduct>, connection: DbConn) -> Result<Json<PharmacyPurchaseProduct>, Status> {
    pharmacies_purchases_products::repository::update(id, pharmacy_purchase_product.into_inner(), &connection)
        .map(|pharmacy_purchase_product| Json(pharmacy_purchase_product))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match pharmacies_purchases_products::repository::get(id, &connection) {
        Ok(_) => pharmacies_purchases_products::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
