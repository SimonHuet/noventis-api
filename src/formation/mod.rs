#[get("/all")]
fn all() -> String {
    format!("Hello, year old named!")
}

#[get("/<id>")]
fn by_id(id:i32) -> String {

    format!("id : {}", id)
}

pub fn create_routes(){
    rocket::ignite().mount("/formation",
     routes![
         all,
         by_id
         ]).launch();
}