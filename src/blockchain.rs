use crate::block::Block;
use crate::transaction::Transaction;
use crate::storage::Storage;

pub struct Blockchain {
    pub storage: Storage,
}

impl Blockchain {
    pub fn new(path: &str) -> Self {
        let storage = Storage::init(path);
        Self { storage }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Block {
        let prev_hash = self.storage.last_hash();
        let block = Block::new(transactions, prev_hash);
        self.storage.save_block(&block);
        block
    }

    pub fn chain(&self) -> Vec<Block> {
        self.storage.load_chain()
    }

    pub fn verify_chain(&self) -> bool {
        let chain = self.chain();
        for i in 1..chain.len() {
            if chain[i].previous_hash != chain[i - 1].hash {
                return false;
            }
        }
        true
    }
}
