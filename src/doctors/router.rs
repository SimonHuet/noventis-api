use doctors;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![doctors::handler::all,
            doctors::handler::get,
            doctors::handler::post,
            doctors::handler::put,
            doctors::handler::delete]
}