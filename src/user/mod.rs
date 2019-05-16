use rocket_contrib::{ json, json::JsonValue, json::Json};
mod user;
use user::{User};

#[get("/all")]
fn all() -> JsonValue {
    json!({
        "id": 83,
        "values": [1, 2, 3, 4]
    })
}

#[post("/", data = "<user>")]
fn create(user: Json<User>)-> Json<User> {
    user
}

#[get("/<id>")]
fn by_id(id:i32) -> String {

    format!("id : {}", id)
}

pub fn create_routes(){
    rocket::ignite().mount("/user",
     routes![
         all,
         by_id,
         create
         ]).launch();
}