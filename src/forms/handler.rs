use connection::DbConn;
use diesel::result::Error;
use std::env;
use forms;
use forms::Form;
use rocket::http::Status;
use rocket::response::status;
use forms::repository::InsertableForm;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Form>>, Status> {
    forms::repository::all(&connection)
        .map(|form| Json(form))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Form>, Status> {
    forms::repository::get(id, &connection)
        .map(|form| Json(form))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<form>")]
pub fn post(form: Json<InsertableForm>, connection: DbConn) -> Result<status::Created<Json<Form>>, Status> {
    forms::repository::insert(form.into_inner(), &connection)
        .map(|form| form_created(form))
        .map_err(|error| error_status(error))
}

fn form_created(form: Form) -> status::Created<Json<Form>> {
    
    status::Created(
        format!("{host}:{port}/form/{id}", host = host(), port = port(), id=form.id).to_string(),
        Some(Json(form)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<form>")]
pub fn put(id: i32, form: Json<InsertableForm>, connection: DbConn) -> Result<Json<Form>, Status> {
    forms::repository::update(id, form.into_inner(), &connection)
        .map(|form| Json(form))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match forms::repository::get(id, &connection) {
        Ok(_) => forms::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
