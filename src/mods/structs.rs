use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String
}