#[derive( Serialize, Deserialize)]
pub struct Doctor {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub age: i32
}