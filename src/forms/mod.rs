#![allow(proc_macro_derive_resolution_fallback)]
use schema::forms;

use serde_json::Value;

pub mod handler;
pub mod repository;
pub mod router;
#[cfg(test)]
pub mod test;

#[derive(Queryable,Identifiable,AsChangeset, Serialize, Deserialize, Associations)]
pub struct Form {
    pub id: i32,
    pub created_at: String,
    pub form: Value 
}