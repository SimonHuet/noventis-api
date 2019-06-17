use formations;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![formations::handler::all,
            formations::handler::get,
            formations::handler::post,
            formations::handler::put,
            formations::handler::delete]
}