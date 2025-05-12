use sled::{Db};
use crate::block::Block;
use serde_json;

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn init(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open database");
        Self { db }
    }

    pub fn save_block(&mut self, block: &Block) {
        self.db
            .insert(block.hash.clone(), serde_json::to_vec(block).unwrap())
            .expect("Failed to write block");
        self.db.insert("last_hash", block.hash.as_bytes()).unwrap();
    }

    pub fn load_chain(&self) -> Vec<Block> {
        let mut chain = Vec::new();
        for entry in self.db.iter() {
            let (key, value) = entry.unwrap();
            let key_str = String::from_utf8(key.to_vec()).unwrap();
            if key_str == "last_hash" { continue; }
            let block: Block = serde_json::from_slice(&value).unwrap();
            chain.push(block);
        }
        chain.sort_by(|a, b| a.index.cmp(&b.index));
        chain
    }

    pub fn last_hash(&self) -> String {
        self.db.get("last_hash")
            .unwrap()
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
            .unwrap_or_default()
    }
}
