use pharmacies_sales_products;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![pharmacies_sales_products::handler::all,
            pharmacies_sales_products::handler::get,
            pharmacies_sales_products::handler::post,
            pharmacies_sales_products::handler::put,
            pharmacies_sales_products::handler::delete]
}