use sha2::{Digest, Sha256};
use std::collections::HashMap;

fn hash_bytes(b: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b);
    hasher.finalize().into()
}

pub struct Merge {
    pub left_key: String,
    pub right_key: String,
    pub probability: f32,
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

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        DisjointSet {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x])
        }
        self.parent[x]
    }

    fn link(&mut self, x: usize, y: usize) {
        if self.rank[x] > self.rank[y] {
            self.parent[y] = x;
        } else {
            self.parent[x] = y;
            if self.rank[x] == self.rank[y] {
                self.rank[y] += 1;
            }
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        self.link(x_root, y_root);
    }

    fn get_components(&mut self) -> Vec<Vec<usize>> {
        let mut components: HashMap<usize, Vec<usize>> = HashMap::new();
        for x in 0..self.parent.len() {
            let root = self.find(x);
            components.entry(root).or_insert(Vec::new()).push(x);
        }
        components.into_values().collect()
    }
}

// fn merges_to_clusters(merges: Vec<Merge>) -> Vec<Cluster> {
//     ...
// }

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

    #[test]
    fn connected_components() {
        let mut djs = DisjointSet::new(100);
        djs.union(0, 1);
        assert_eq!(djs.get_components().len(), 99);
    }
}
