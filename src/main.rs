mod helpers;
mod services;

use helpers::{block::Block, blockchain::Blockchain};
use services::hash::Hash;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let blockchain = Blockchain::new(7);
    let index = blockchain.chain.len();
    let transactions: Vec<String> = [
        "Alice pays Bob 5 BTC",
        "Bob pays Charlie 3 BTC",
        "Charlie pays Dave 1 BTC",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect();

    let previous_hash = "genesis".to_string();
    let difficulty = blockchain.difficulty;
    let mut block = Block::new(index as u32, transactions, previous_hash, difficulty);

    let hash = Hash::new(block.generate_message());
    block.set_hash(hash);
    block.mine();

    // push to blockchain

    // give reward

    let duration = start.elapsed();
    print!("{:?}", block);
    println!("Mining took: {:?}", duration);
}
