use pharmacies;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![pharmacies::handler::all,
            pharmacies::handler::get,
            pharmacies::handler::post,
            pharmacies::handler::put,
            pharmacies::handler::delete]
}