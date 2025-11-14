pub struct Cluster {
    pub id: u64,
    pub hash: [u8; 32],
    pub probability: f32,
}

#[cfg(test)]
mod tests {
    use crate::Cluster;
    use sha2::{Digest, Sha256};

    #[test]
    fn make_cluster() {
        let mut hasher = Sha256::new();
        hasher.update(b"hello world");
        let hash: [u8; 32] = hasher.finalize().into();
        Cluster {
            id: 12,
            hash: hash,
            probability: 0.82,
        };
    }
}
