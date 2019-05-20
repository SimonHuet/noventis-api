#![feature(plugin, proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate r2d2;
#[macro_use] extern crate r2d2_diesel;

pub mod models;
pub mod schema;
pub mod routes;
pub mod dbconnection;

use routes::root;
use routes::doctor;
use routes::formation;

fn main() {
   //doctor::create_routes();
   rocket::ignite().mount("/api", 
        routes![
                root::index,
                doctor::all,
                doctor::post,
                doctor::by_id,
                formation::all,
                formation::by_id
                ]).launch();
}