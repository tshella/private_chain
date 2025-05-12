use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let index = Utc::now().timestamp() as u64;
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            index,
            timestamp: timestamp.clone(),
            transactions,
            previous_hash,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let data = format!("{:?}{:?}{:?}", self.index, self.timestamp, self.transactions);
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}
