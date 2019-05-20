#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "Formation"]
pub struct formation {
    pub id: Option<i32>,
    pub name: String,
    pub description: String
}