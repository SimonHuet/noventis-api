use forms;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![forms::handler::all,
            forms::handler::get,
            forms::handler::post,
            forms::handler::put,
            forms::handler::delete]
}