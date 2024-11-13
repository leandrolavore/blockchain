use sha2::{Digest, Sha256};
pub struct Hash;

impl Hash {
    pub fn new(hash_message: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(hash_message);
        let result = hasher.finalize();

        format!("{:x}", result)
    }
}
