#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod doctors;
mod formations;
mod doctors_formations;
mod schema;
mod connection;

fn main() {
    dotenv().ok();
    
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/doctors",
               doctors::router::get_routes()
        )
        .mount("/formations",
               formations::router::get_routes()
        )
        .mount("/doctors-formations",
              doctors_formations::router::get_routes()
       )
        .launch();
}
