use rocket_contrib::json::{JsonValue , Json };
mod doctor;
use doctor::{Doctor};

#[get("/all")]
fn all() -> JsonValue {
    json!({
        "id": 83,
        "values": [1, 2, 3, 4]
    })
}

#[post("/", data = "<doctor>")]
fn create(doctor: Json<Doctor>)-> Json<Doctor> {
    doctor
}

#[get("/<id>")]
fn by_id(id:i32) -> String {

    format!("id : {}", id)
}

pub fn create_routes(){
    rocket::ignite().mount("/doctor",
     routes![
         all,
         by_id,
         create
         ]).launch();
}