use crate::hash::hash_bytes;

#[derive(Debug, Clone)]
pub struct Cluster {
    pub hash: [u8; 32],
    pub probability: Option<f32>,
    pub leaves: Vec<[u8; 32]>,
}

impl Cluster {
    pub fn combine(clusters: Vec<&Self>, probability: f32) -> Option<Self> {
        if clusters.len() == 0 {
            return None;
        };
        if clusters.len() == 1 {
            let mut clone = clusters[0].clone();
            clone.probability = Some(probability);
            return Some(clone);
        }
        let concat_hashes: Vec<u8> = clusters.iter().flat_map(|c| c.hash).collect();
        let concat_leaves: Vec<_> = clusters
            .iter()
            .flat_map(|c| c.leaves.iter().cloned())
            .collect();

        Some(Cluster {
            hash: hash_bytes(&concat_hashes),
            probability: Some(probability),
            leaves: concat_leaves,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::hash_bytes;

    #[test]
    fn make_cluster() {
        let c1 = Cluster {
            hash: hash_bytes(b"hello world"),
            probability: Some(0.82),
            leaves: vec![hash_bytes(b"hello world")],
        };

        let c2 = Cluster {
            hash: hash_bytes(b"ciao mondo"),
            probability: Some(0.50),
            leaves: vec![hash_bytes(b"ciao mondo")],
        };

        let combined = Cluster::combine(vec![&c1, &c2], 0.75).unwrap();
        assert_eq!(combined.probability.unwrap(), 0.75);
        assert_ne!(combined.hash, c1.hash);
        assert_ne!(combined.hash, c2.hash);
    }
}
