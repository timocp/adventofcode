// Disjoint set / union find data structure

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            // each element begins as the only item in its own set
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn find(&mut self, n: usize) -> usize {
        if self.parent[n] == n {
            n
        } else {
            self.parent[n] = self.find(self.parent[n]);
            self.parent[n]
        }
    }

    // returns true if a merge happened
    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let iset = self.find(i);
        let jset = self.find(j);

        if iset == jset {
            return false;
        }

        if self.size[iset] < self.size[jset] {
            self.parent[iset] = jset;
            self.size[jset] += self.size[iset];
        } else {
            self.parent[jset] = iset;
            self.size[iset] += self.size[jset];
        }

        true
    }

    pub fn is_root(&self, n: usize) -> bool {
        self.parent[n] == n
    }

    // Returns the Some(size) for root nodes, None for non-root nodes
    pub fn size_of(&self, n: usize) -> Option<usize> {
        if self.is_root(n) {
            Some(self.size[n])
        } else {
            // size[n] is not meaningful for merged sets
            None
        }
    }
}
