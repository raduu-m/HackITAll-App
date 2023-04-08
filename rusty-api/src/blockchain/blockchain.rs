use std::fmt::{Display, Formatter, Result};
use sha2::{Digest, Sha256};
// A block in the blockchain
pub struct Block {
    index: usize,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
}

impl Block {
    // Calculate the SHA-256 hash of the block
    pub fn calculate_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", self.index, self.timestamp, self.data, self.prev_hash));
        self.hash = format!("{:x}", hasher.finalize());
    }
}

impl Display for Block {
    // Implement Display for Block to print out its data
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Block {}: {} at {} with data: {}", self.index, self.hash, self.timestamp, self.data)
    }
}

pub struct Blockchain {
    blocks: Vec<Block>,
}


impl Blockchain {
    // Add a block to the chain
    pub fn add_block(&mut self, data: String) {
        let prev_hash = match self.blocks.last() {
            Some(block) => block.hash.clone(),
            None => "0".to_owned(),
        };
        let index = self.blocks.len();
        let timestamp = chrono::Utc::now().timestamp();
        let mut block = Block { index, timestamp, data, prev_hash, hash: "".to_owned() };
        block.calculate_hash();
        self.blocks.push(block);
    }

    // Validate the blockchain
    pub fn is_valid(&self) -> bool {
        let mut last_hash = "0".to_owned();
        for block in &self.blocks {
            if block.prev_hash != last_hash {
                return false;
            }
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}{}{}", block.index, block.timestamp, block.data, block.prev_hash));
            let hash = format!("{:x}", hasher.finalize());
            if hash != block.hash {
                return false;
            }
            last_hash = block.hash.clone();
        }
        true
    }
}