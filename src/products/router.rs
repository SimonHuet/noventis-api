use products;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![products::handler::all,
            products::handler::get,
            products::handler::post,
            products::handler::put,
            products::handler::delete]
}