use sha2::{Digest, Sha256};

pub fn hash_bytes(b: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b);
    hasher.finalize().into()
}
