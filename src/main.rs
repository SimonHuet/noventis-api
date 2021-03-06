#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;

mod doctors;
mod formations;
mod doctors_formations;
mod forms;
mod pharmacies;
mod products;
mod pharmacies_purchases_products;
mod pharmacies_sales_products;
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
       .mount("/forms",
              forms::router::get_routes()
       )
       .mount("/pharmacies",
              pharmacies::router::get_routes()
       )
       .mount("/products",
              products::router::get_routes()
       ).mount("/purchases",
              pharmacies_purchases_products::router::get_routes()
       ).mount("/sales",
              pharmacies_sales_products::router::get_routes()
       )
       .launch();
}
