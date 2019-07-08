use pharmacies_purchases_products;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![pharmacies_purchases_products::handler::all,
            pharmacies_purchases_products::handler::get,
            pharmacies_purchases_products::handler::post,
            pharmacies_purchases_products::handler::put,
            pharmacies_purchases_products::handler::delete]
}