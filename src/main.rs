#![feature(plugin, proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;

mod doctor;

fn main() {

   doctor::create_routes()
}