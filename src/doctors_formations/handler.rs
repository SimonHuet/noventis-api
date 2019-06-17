use connection::DbConn;
use diesel::result::Error;
use std::env;
use doctors_formations;
use doctors_formations::DoctorFormation;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use doctors_formations::repository::InsertableDoctorFormation;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<DoctorFormation>>, Status> {
    doctors_formations::repository::all(&connection)
        .map(|doctors_formations| Json(doctors_formations))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<DoctorFormation>, Status> {
    doctors_formations::repository::get(id, &connection)
        .map(|formation| Json(formation))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<doctorformation>")]
pub fn post(doctorformation: Json<InsertableDoctorFormation>, connection: DbConn) -> Result<status::Created<Json<DoctorFormation>>, Status> {
    doctors_formations::repository::insert(doctorformation.into_inner(), &connection)
        .map(|doctorformation| doctorformation_created(doctorformation))
        .map_err(|error| error_status(error))
}

fn doctorformation_created(doctorformation: DoctorFormation) -> status::Created<Json<DoctorFormation>> {
    status::Created(
        format!("{host}:{port}/formation/{id}", host = host(), port = port(), id=doctorformation.id ).to_string(),
        Some(Json(doctorformation)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<doctorformation>")]
pub fn put(id: i32, doctorformation: Json<InsertableDoctorFormation>, connection: DbConn) -> Result<Json<DoctorFormation>, Status> {
    doctors_formations::repository::update(id, doctorformation.into_inner(), &connection)
        .map(|doctorformation| Json(doctorformation))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match doctors_formations::repository::get(id, &connection) {
        Ok(_) => doctors_formations::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
