use serde::{Serialize, Deserialize};
use chrono::prelude::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct DbConfig {
    pub path: String,
    pub cache_capacity: u64,
    pub flush_every_ms: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlertData<T> {
    alert_message: T,
    id: u64,
    created_by: DateTime<Utc>,
}

impl<T> AlertData<T> {
    pub fn new(alert_message: T) -> AlertData<T>{
        AlertData {
            id: 1,
            created_by: Utc::now(),
            alert_message,
        }
    }
}