use serde::{Deserialize, Serialize};

use sciter::{ Element };

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Person {
    pub id: i32,
    pub name: String
}


pub struct UIEvents {
    pub root: Option<Element>,
}