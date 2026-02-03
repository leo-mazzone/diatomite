use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    hash::{self, Hash},
};

fn hash_bytes(b: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b);
    hasher.finalize().into()
}

pub struct Merge {
    pub left: usize,
    pub right: usize,
    pub probability: f32,
}

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

fn merges_to_clusters(mut merges: Vec<Merge>, clusters: &Vec<Cluster>) -> Vec<Cluster> {
    let mut djs = DisjointSet::new(clusters.len());
    let mut new_clusters = Vec::new();

    if merges.is_empty() {
        return new_clusters;
    }

    merges.sort_unstable_by(|a, b| b.probability.total_cmp(&a.probability));
    let mut prev_p = merges[0].probability;

    for m in &merges {
        if prev_p != m.probability {
            // Output all current components at the previous probability
            for component in djs.get_components() {
                let leaf_clusters: Vec<&Cluster> =
                    component.into_iter().map(|leaf| &clusters[leaf]).collect();
                if leaf_clusters.len() > 1 {
                    new_clusters.push(Cluster::combine(leaf_clusters, prev_p).unwrap());
                }
            }
            prev_p = m.probability;
        }
        djs.union(m.left, m.right)
    }

    // Output final state after all merges
    for component in djs.get_components() {
        let leaf_clusters: Vec<&Cluster> = component.iter().map(|&leaf| &clusters[leaf]).collect();
        if leaf_clusters.len() > 1 {
            new_clusters.push(Cluster::combine(leaf_clusters, prev_p).unwrap());
        }
    }

    new_clusters
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::*;

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

    #[test]
    fn connected_components() {
        let mut djs = DisjointSet::new(100);
        djs.union(0, 1);
        assert_eq!(djs.get_components().len(), 99);
    }

    #[test]
    fn no_merges() {
        let clusters = vec![Cluster {
            hash: hash_bytes(b"a"),
            probability: None,
            leaves: vec![hash_bytes(b"a")],
        }];

        let merges = vec![];

        let new_clusters = merges_to_clusters(merges, &clusters);
        assert_eq!(new_clusters.len(), 0);
    }

    #[test]
    fn merge_clusters_determin() {
        let clusters = vec![
            Cluster {
                hash: hash_bytes(b"a"),
                probability: None,
                leaves: vec![hash_bytes(b"a")],
            },
            Cluster {
                hash: hash_bytes(b"b"),
                probability: None,
                leaves: vec![hash_bytes(b"b")],
            },
            Cluster {
                hash: hash_bytes(b"c"),
                probability: None,
                leaves: vec![hash_bytes(b"c")],
            },
            Cluster {
                hash: hash_bytes(b"d"),
                probability: None,
                leaves: vec![hash_bytes(b"d")],
            },
        ];

        let merges = vec![
            Merge {
                left: 0,
                right: 1,
                probability: 1.0,
            },
            Merge {
                left: 0,
                right: 2,
                probability: 1.0,
            },
        ];
        let new_clusters = merges_to_clusters(merges, &clusters);
        assert_eq!(new_clusters.len(), 1);

        let expected_merged: HashSet<_> =
            [hash_bytes(b"a"), hash_bytes(b"b"), hash_bytes(b"c")].into();

        let merged_cluster = new_clusters
            .iter()
            .find(|c| {
                let leaves: HashSet<_> = c.leaves.iter().cloned().collect();
                leaves == expected_merged
            })
            .unwrap();

        assert_eq!(merged_cluster.probability, Some(1.0));
    }

    #[test]
    fn merge_clusters_prob() {
        let clusters = vec![
            Cluster {
                hash: hash_bytes(b"a"),
                probability: None,
                leaves: vec![hash_bytes(b"a")],
            },
            Cluster {
                hash: hash_bytes(b"b"),
                probability: None,
                leaves: vec![hash_bytes(b"b")],
            },
            Cluster {
                hash: hash_bytes(b"c"),
                probability: None,
                leaves: vec![hash_bytes(b"c")],
            },
            Cluster {
                hash: hash_bytes(b"d"),
                probability: None,
                leaves: vec![hash_bytes(b"d")],
            },
        ];
        let merges = vec![
            Merge {
                left: 0,
                right: 1,
                probability: 1.0,
            },
            Merge {
                left: 2,
                right: 3,
                probability: 1.0,
            },
            Merge {
                left: 1,
                right: 2,
                probability: 0.85,
            },
        ];
        let new_clusters = merges_to_clusters(merges, &clusters);
        assert_eq!(new_clusters.len(), 3);

        let expected_merged_100_1: HashSet<_> = [hash_bytes(b"a"), hash_bytes(b"b")].into();
        let expected_merged_100_2: HashSet<_> = [hash_bytes(b"c"), hash_bytes(b"d")].into();
        let expected_merged_85: HashSet<_> = [
            hash_bytes(b"a"),
            hash_bytes(b"b"),
            hash_bytes(b"c"),
            hash_bytes(b"d"),
        ]
        .into();

        let mut merged_100_1 = None;
        let mut merged_100_2 = None;
        let mut merged_85 = None;

        for c in &new_clusters {
            let leaf_set: HashSet<_> = c.leaves.iter().cloned().collect();
            if leaf_set == expected_merged_100_1 {
                merged_100_1 = Some(c);
            }
            if leaf_set == expected_merged_100_2 {
                merged_100_2 = Some(c);
            }
            if leaf_set == expected_merged_85 {
                merged_85 = Some(c);
            }
        }

        let merged_100_1 = merged_100_1.unwrap();
        let merged_100_2 = merged_100_2.unwrap();
        let merged_85 = merged_85.unwrap();

        assert_eq!(merged_100_1.probability, Some(1.0));
        assert_eq!(merged_100_2.probability, Some(1.0));
        assert_eq!(merged_85.probability, Some(0.85));
    }
}
