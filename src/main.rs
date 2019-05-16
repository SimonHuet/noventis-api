#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{ Json ,value};

mod user;
use user::{User};

#[post("/", data = "<user>")]
fn create(user: Json<User>) -> Json<User> {
    user
}

#[get("/")]
fn read() -> Json<Value> {
    Json(json!([
        "User 1", 
        "User 2"
    ]))
}
/*
#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>) -> Json<User> {
    user
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<Value> {
    Json(json!({"status": "ok"}))
}*/

fn main() {
    rocket::ignite()
        .mount("/user", routes![create /*,update, delete*/])
        .mount("/users", routes![read])
        .launch();
}