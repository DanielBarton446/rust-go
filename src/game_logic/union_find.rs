use std::cmp::Ordering;

#[derive(Debug)]
pub struct UnionFind {
    pub parent: Vec<usize>,
    rank: Vec<usize>, // optimizes "tree" height for quicker lookups.
    size: Vec<usize>, // keep track of size of the chains? do we need this?
}

impl UnionFind {
    pub fn new(board_size: usize) -> UnionFind {
        let mut parent = Vec::with_capacity(board_size);
        let rank = vec![0; board_size];
        let size = vec![1; board_size];

        for i in 0..board_size {
            parent.push(i);
        }

        UnionFind { parent, rank, size }
    }

    fn find(&mut self, mut target: usize) -> usize {
        while target != self.parent[target] {
            self.parent[target] = self.parent[self.parent[target]];
            target = self.parent[target];
        }
        target
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        // self.parent[root_x] = root_y;
        // self.size[root_y] += self.size[root_x];

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
                self.size[root_x] += self.size[root_y];
            }
        }
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

mod tests {

    #[test]
    fn test_union() {
        use crate::union_find::UnionFind;

        let mut uf = UnionFind::new(3 * 3);
        uf.parent = vec![0, 0, 2, 3, 4, 5, 6, 6, 6];
        uf.size = vec![2, 1, 1, 1, 1, 1, 3, 1, 1];
        uf.union(4, 1);
        let expected = vec![4, 0, 2, 3, 4, 5, 6, 6, 6];
        assert_eq!(uf.parent, expected);
        assert_eq!(uf.size[4], 3);

        uf.union(4, 7);
        let expected = vec![4, 0, 2, 3, 4, 5, 4, 6, 6];
        assert_eq!(uf.parent, expected);
        assert_eq!(uf.size[4], 6);
    }
}
