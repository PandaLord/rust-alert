use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub id: i32,
    pub message: String,
    pub created_on: String,
}

impl Alert {
    pub fn new(id: i32, msg: String) -> Self {
        Alert {
            id,
            message: msg,
            created_on: "test".to_owned(),
        }
    }
}
