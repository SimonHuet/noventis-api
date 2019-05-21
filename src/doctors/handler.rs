use connection::DbConn;
use diesel::result::Error;
use std::env;
use doctors;
use doctors::Doctor;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Doctor>>, Status> {
    doctors::repository::all(&connection)
        .map(|doctors| Json(doctors))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Doctor>, Status> {
    doctors::repository::get(id, &connection)
        .map(|doctor| Json(doctor))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<doctor>")]
pub fn post(doctor: Json<Doctor>, connection: DbConn) -> Result<status::Created<Json<Doctor>>, Status> {
    doctors::repository::insert(doctor.into_inner(), &connection)
        .map(|doctor| doctor_created(doctor))
        .map_err(|error| error_status(error))
}

fn doctor_created(doctor: Doctor) -> status::Created<Json<Doctor>> {
    
    status::Created(
        format!("{host}:{port}/doctor/{id}", host = host(), port = port(), id=doctor.id).to_string(),
        Some(Json(doctor)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<doctor>")]
pub fn put(id: i32, doctor: Json<Doctor>, connection: DbConn) -> Result<Json<Doctor>, Status> {
    doctors::repository::update(id, doctor.into_inner(), &connection)
        .map(|doctor| Json(doctor))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match doctors::repository::get(id, &connection) {
        Ok(_) => doctors::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
