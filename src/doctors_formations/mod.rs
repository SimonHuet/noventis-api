use schema::doctors_formations;

use doctors::Doctor;
use formations::Formation;

pub mod handler;
pub mod repository;
pub mod router;

#[derive(Identifiable, Queryable, Associations, Serialize , Deserialize)]
#[belongs_to(Doctor)]
#[belongs_to(Formation)]
#[table_name = "doctors_formations"]
pub struct DoctorFormation{
    pub id : i32,
    pub doctor_id : i32,
    pub formation_id : i32
}