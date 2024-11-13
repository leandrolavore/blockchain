use std::time::{SystemTime, UNIX_EPOCH};

use crate::services::hash::Hash;

#[derive(Debug)]
pub struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<String>,
    hash: String,
    previous_hash: String,
    nonce: u64,
    difficulty: u32,
}

impl Block {
    pub fn new(
        index: u32,
        transactions: Vec<String>,
        previous_hash: String,
        difficulty: u32,
    ) -> Block {
        Block {
            index,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            transactions,
            hash: "".to_string(),
            previous_hash,
            nonce: 0,
            difficulty,
        }
    }

    pub fn generate_message(&self) -> String {
        let transactions_string = self.transactions.join(", ");
        format!(
            "{}|{}|{:?}|{}|{}",
            self.index, self.timestamp, transactions_string, self.nonce, self.difficulty
        )
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn mine(&mut self) {
        let mut iterations = 0;

        while !self.hash.starts_with(&"0".repeat(self.difficulty as usize)) {
            self.nonce += 1;
            iterations += 1;
            let message = self.generate_message();
            self.hash = Hash::new(message);
        }

        println!("It took the mining {:?} iterations", iterations)
    }
}
