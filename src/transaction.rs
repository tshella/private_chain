use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub item: String,
    pub timestamp: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, item: String) -> Self {
        Self {
            sender,
            receiver,
            item,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
