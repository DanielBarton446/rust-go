use std::{cmp::Ordering, collections::HashSet};

#[derive(Debug)]
pub struct UnionFind {
    pub parent: Vec<usize>,
    rank: Vec<usize>, // optimizes "tree" height for quicker lookups.
    size: Vec<usize>, // keep track of size of the chains? do we need this?
    pub liberties: Vec<HashSet<usize>>, // liberties positions of the chain the current index
                      // represents.
}

impl UnionFind {
    pub fn new(board_size: usize) -> UnionFind {
        let mut parent = Vec::with_capacity(board_size);
        let rank = vec![0; board_size];
        let size = vec![1; board_size];
        let liberties = vec![HashSet::new(); board_size];

        for i in 0..board_size {
            parent.push(i);
        }

        UnionFind {
            parent,
            rank,
            size,
            liberties,
        }
    }

    fn find(&mut self, mut target: usize) -> usize {
        while target != self.parent[target] {
            self.parent[target] = self.parent[self.parent[target]];
            target = self.parent[target];
        }
        target
    }

    pub fn initialize_liberties_of_pos(&mut self, index: usize, libs: Vec<usize>) {
        // only allow for this function to be called when the position is empty.
        // If the position has liberties set already, its in a state of altercation and
        // must not be updated. If we allow updates, we risk allowing overwriting of
        // valid and currenlty in use chains/stones
        //
        // We also assume that libs is a valid vector of liberties being provided from the caller.
        // This should be checked todo!()
        if !self.liberties[index].is_empty() {
            todo!("This should return an error and error softly.")
        }

        for lib in libs {
            self.liberties[index].insert(lib);
        }
    }

    pub fn remove_liberty_from_chain(&mut self, victim: usize, perpetrator: usize) {
        let root_victim = self.find(victim);
        self.liberties[root_victim].remove(&perpetrator);
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        // self.parent[root_x] = root_y;
        // self.size[root_y] += self.size[root_x];
        //// capture liberties and CLEAR self.liberties[root_x] with empty vec.
        // let old_libs = std::mem::take(&mut self.liberties[root_x]);
        // self.liberties[root_y].extend(old_libs);

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];

                // capture liberties and CLEAR self.liberties[root_x] with empty vec.
                let old_libs = std::mem::take(&mut self.liberties[root_x]);
                self.liberties[root_y].extend(old_libs);
                // remove the two stones from the liberties
                self.liberties[root_y].remove(&x);
                self.liberties[root_y].remove(&y);
            }
            Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];

                // capture liberties and CLEAR self.liberties[root_x] with empty vec.
                let old_libs = std::mem::take(&mut self.liberties[root_y]);
                self.liberties[root_x].extend(old_libs);
                // remove the two stones from the liberties
                self.liberties[root_x].remove(&x);
                self.liberties[root_x].remove(&y);
            }
            Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
                self.size[root_x] += self.size[root_y];

                // capture liberties and CLEAR self.liberties[root_x] with empty vec.
                let old_libs = std::mem::take(&mut self.liberties[root_y]);
                self.liberties[root_x].extend(old_libs);
                // remove the two stones from the liberties
                self.liberties[root_x].remove(&x);
                self.liberties[root_x].remove(&y);
            }
        }
    }

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub(crate) fn no_liberties(&self, adjacent_index: usize) -> bool {
        self.liberties[adjacent_index].is_empty()
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

    #[test]
    fn liberties_unioned_stones() {
        use crate::union_find::UnionFind;
        use std::collections::HashSet;

        let mut uf = UnionFind::new(3 * 3);
        uf.initialize_liberties_of_pos(0, vec![1, 3]);
        uf.initialize_liberties_of_pos(1, vec![0, 2, 4]);
        uf.union(0, 1);
        let expected_libs = HashSet::from_iter(vec![2, 3, 4]);
        assert_eq!(uf.liberties[0], expected_libs);
    }

    #[test]
    fn liberties_union_two_chains() {
        use crate::union_find::UnionFind;
        use std::collections::HashSet;

        let mut uf = UnionFind::new(3 * 3);
        uf.initialize_liberties_of_pos(0, vec![1, 3]);
        uf.initialize_liberties_of_pos(1, vec![0, 2, 4]);
        uf.initialize_liberties_of_pos(2, vec![1, 5]);
        uf.union(0, 1);
        uf.union(1, 2);
        let expected_libs = HashSet::from_iter(vec![3, 4, 5]);
        assert_eq!(uf.liberties[0], expected_libs);
    }
}
