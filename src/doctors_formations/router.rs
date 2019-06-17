use doctors_formations;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![doctors_formations::handler::all,
            doctors_formations::handler::get,
            doctors_formations::handler::post,
            doctors_formations::handler::put,
            doctors_formations::handler::delete]
}