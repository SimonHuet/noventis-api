#[get("/formation/all")]
pub fn all() -> String {
    format!("Hello, year old named!")
}

#[get("/formation/<id>")]
pub fn by_id(id:i32) -> String {

    format!("id : {}", id)
}
