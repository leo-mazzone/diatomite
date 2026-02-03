use std::collections::HashMap;

pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<u32>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
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

    pub fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        self.link(x_root, y_root);
    }

    pub fn get_components(&mut self) -> Vec<Vec<usize>> {
        let mut components: HashMap<usize, Vec<usize>> = HashMap::new();
        for x in 0..self.parent.len() {
            let root = self.find(x);
            components.entry(root).or_insert(Vec::new()).push(x);
        }
        components.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connected_components() {
        let mut djs = DisjointSet::new(100);
        djs.union(0, 1);
        assert_eq!(djs.get_components().len(), 99);
    }
}
