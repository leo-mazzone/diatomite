use sha2::{Digest, Sha256};

fn hash_bytes(b: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b);
    hasher.finalize().into()
}

pub struct Cluster {
    pub id: u64,
    pub hash: [u8; 32],
    pub probability: Option<f32>,
}

impl Cluster {
    pub fn combine(id: u64, clusters: Vec<Self>, probability: f32) -> Self {
        let concat_hashes: Vec<u8> = clusters.iter().flat_map(|c| c.hash).collect();

        Cluster {
            id: id,
            hash: hash_bytes(&concat_hashes),
            probability: Some(probability),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use base64::{Engine as _, engine::general_purpose};

    #[test]
    fn make_cluster() {
        let c1 = Cluster {
            id: 1,
            hash: hash_bytes(b"hello world"),
            probability: Some(0.82),
        };

        let c2 = Cluster {
            id: 2,
            hash: hash_bytes(b"ciao mondo"),
            probability: Some(0.50),
        };

        let combined = Cluster::combine(3, vec![c1, c2], 0.75);
        println!("{}", general_purpose::STANDARD.encode(combined.hash));
    }
}
