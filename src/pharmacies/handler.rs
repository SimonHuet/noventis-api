use connection::DbConn;
use diesel::result::Error;
use std::env;
use pharmacies;
use pharmacies::Pharmacy;
use rocket::http::Status;
use rocket::response::status;
use pharmacies::repository::InsertablePharmacy;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Pharmacy>>, Status> {
    pharmacies::repository::all(&connection)
        .map(|pharmacy| Json(pharmacy))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Pharmacy>, Status> {
    pharmacies::repository::get(id, &connection)
        .map(|pharmacy| Json(pharmacy))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<pharmacy>")]
pub fn post(pharmacy: Json<InsertablePharmacy>, connection: DbConn) -> Result<status::Created<Json<Pharmacy>>, Status> {
    pharmacies::repository::insert(pharmacy.into_inner(), &connection)
        .map(|pharmacy| pharmacy_created(pharmacy))
        .map_err(|error| error_status(error))
}

fn pharmacy_created(pharmacy: Pharmacy) -> status::Created<Json<Pharmacy>> {
    
    status::Created(
        format!("{host}:{port}/pharmacay/{id}", host = host(), port = port(), id=pharmacy.id).to_string(),
        Some(Json(pharmacy)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<pharmacy>")]
pub fn put(id: i32, pharmacy: Json<InsertablePharmacy>, connection: DbConn) -> Result<Json<Pharmacy>, Status> {
    pharmacies::repository::update(id, pharmacy.into_inner(), &connection)
        .map(|pharmacy| Json(pharmacy))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match pharmacies::repository::get(id, &connection) {
        Ok(_) => pharmacies::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
