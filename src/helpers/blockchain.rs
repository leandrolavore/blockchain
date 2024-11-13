use super::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub mempool: Vec<String>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Blockchain {
        Blockchain {
            chain: Vec::new(),
            difficulty,
            mempool: Vec::new(),
        }
    }
}
