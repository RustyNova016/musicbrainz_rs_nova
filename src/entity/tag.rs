use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Tag {
    pub name: String,
    pub count: Option<i32>,
    pub score: Option<i32>,
}
