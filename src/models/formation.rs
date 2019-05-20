#[derive(Queryable, Serialize, Deserialize)]
pub struct Formation {
    pub name: String,
    pub description: String
}
