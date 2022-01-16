use serde::{Serialize, Deserialize};
use chrono::prelude::*;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    pub message: String,
    pub created_on: DateTime<Local>,
}

impl Alert {
    pub fn new(msg: String) -> Self {
        Alert {
            message: msg,
            created_on: Local::now(),
        }
    }
}
