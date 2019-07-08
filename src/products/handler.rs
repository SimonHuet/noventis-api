use connection::DbConn;
use diesel::result::Error;
use std::env;
use products;
use products::Product;
use rocket::http::Status;
use rocket::response::status;
use products::repository::InsertableProduct;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Product>>, Status> {
    products::repository::all(&connection)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Product>, Status> {
    products::repository::get(id, &connection)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<Product>")]
pub fn post(Product: Json<InsertableProduct>, connection: DbConn) -> Result<status::Created<Json<Product>>, Status> {
    products::repository::insert(Product.into_inner(), &connection)
        .map(|product| product_created(product))
        .map_err(|error| error_status(error))
}

fn product_created(Product: Product) -> status::Created<Json<Product>> {
    
    status::Created(
        format!("{host}:{port}/products/{id}", host = host(), port = port(), id=Product.id).to_string(),
        Some(Json(Product)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<Product>")]
pub fn put(id: i32, Product: Json<InsertableProduct>, connection: DbConn) -> Result<Json<Product>, Status> {
    products::repository::update(id, Product.into_inner(), &connection)
        .map(|product| Json(product))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match products::repository::get(id, &connection) {
        Ok(_) => products::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
