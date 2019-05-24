use connection::DbConn;
use diesel::result::Error;
use std::env;
use formations;
use formations::Formation;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use formations::repository::InsertableFormation;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Formation>>, Status> {
    formations::repository::all(&connection)
        .map(|formations| Json(formations))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Formation>, Status> {
    formations::repository::get(id, &connection)
        .map(|formation| Json(formation))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<formation>")]
pub fn post(formation: Json<InsertableFormation>, connection: DbConn) -> Result<status::Created<Json<Formation>>, Status> {
    formations::repository::insert(formation.into_inner(), &connection)
        .map(|formation| person_created(formation))
        .map_err(|error| error_status(error))
}

fn person_created(formation: Formation) -> status::Created<Json<Formation>> {
    status::Created(
        format!("{host}:{port}/formation/{id}", host = host(), port = port(), id=formation.id ).to_string(),
        Some(Json(formation)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<formation>")]
pub fn put(id: i32, formation: Json<InsertableFormation>, connection: DbConn) -> Result<Json<Formation>, Status> {
    formations::repository::update(id, formation.into_inner(), &connection)
        .map(|formation| Json(formation))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match formations::repository::get(id, &connection) {
        Ok(_) => formations::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
