use sha2::{Digest, Sha256};

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    index: u64,
    timestamp: u128,
    data: String,
    proof: u64,
    hash: String,
    previous_hash: String,
}

impl Block {
    /// Create a new block in a specific blockchain index
    pub fn new(index: u64, data: &str, previous_hash: &str) -> Self {
        Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            data: data.to_owned(),
            proof: 0,
            hash: String::default(),
            previous_hash: previous_hash.to_owned(),
        }
    }

    /// Calculate a new hash for the block
    pub fn calculate_hash(
        index: u64,
        timestamp: u128,
        data: &str,
        previous_hash: &str,
        proof: u64,
    ) -> String {
        let data = format!("{}{}{}{}{}", index, timestamp, data, previous_hash, proof);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }

    /// Mine the block
    pub fn mine(&mut self, complexity: usize) {
        loop {
            if !self.hash.starts_with(&"0".repeat(complexity)) {
                self.proof += 1;
                self.hash = Block::calculate_hash(
                    self.index,
                    self.timestamp,
                    &self.data,
                    &self.previous_hash,
                    self.proof,
                );
            } else {
                break;
            }
        }
    }

    /// Get the hash
    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    /// Get the proof
    pub fn proof(&self) -> u64 {
        self.proof
    }

    /// Get the data
    pub fn data(&self) -> String {
        self.data.clone()
    }

    /// Get a timestamp
    pub fn timestamp(&self) -> u128 {
        self.timestamp
    }
}
