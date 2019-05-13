/*#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "user"]*/
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub age: i32
}