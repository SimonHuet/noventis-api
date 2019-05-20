use rocket_contrib::json::{JsonValue , Json };
use rocket::response::status;
use diesel::result::Error;
use rocket::http::Status;
use super::super::models::doctor::Doctor;
use super::super::dbconnection::DbConn;

#[get("/doctor/all")]
pub fn all() -> JsonValue {
    json!({})
}


#[post("/doctor", data = "<doctor>")]
pub fn post(doctor: Json<Doctor>, connection: DbConn) -> Result<status::Created<Json<Doctor>>, Status> {
    doctors::repository::insert(doctor.into_inner(), &connection)
        .map(|doctor| doctor_created(doctor))
}

fn doctor_created(doctor: Doctor) -> status::Created<Json<Doctor>> {
    status::Created(
        format!("id = doctor.id"))
}

#[get("/doctor/<id>")]
pub fn by_id(id:i32) -> String {

    format!("id : {}", id)
}