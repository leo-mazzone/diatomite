mod clusters;
mod disjoint_set;
mod hash;

use crate::clusters::Cluster;
use crate::disjoint_set::DisjointSet;

pub struct Merge {
    pub left: usize,
    pub right: usize,
    pub probability: f32,
}

pub fn merges_to_clusters(mut merges: Vec<Merge>, clusters: &Vec<Cluster>) -> Vec<Cluster> {
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
    use crate::hash::hash_bytes;
    use std::collections::HashSet;

    use super::*;

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
