use std::time::SystemTime;

#[derive( Serialize, Deserialize)]
pub struct Doctor {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: SystemTime
}