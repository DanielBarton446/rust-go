use crate::{game_move::GameMove, stone::Stone};
use std::collections::HashSet;

pub struct Chain {
    pub group: HashSet<(usize, usize)>,
    pub liberties: HashSet<(usize, usize)>,
    pub color: Stone,
}

impl Chain {
    pub fn new(mv: &GameMove, libs: &Vec<(usize, usize)>) -> Self {
        let mut c = Chain {
            group: HashSet::from([mv.pos]),
            liberties: HashSet::new(),
            color: mv.stone,
        };
        c.place_stone_and_update_liberties(mv, libs);
        c
    }

    pub fn place_stone_and_update_liberties(
        &mut self,
        mv: &GameMove,
        libs: &Vec<(usize, usize)>,
    ) -> Result<(), String> {
        if self.color == mv.stone {
            self.add_stone(mv, libs);
        } else {
            self.remove_liberty(mv.pos);
        }
        Ok(())
    }

    pub fn is_dead_chain(&self) -> bool {
        self.liberties.is_empty()
    }

    fn add_stone(&mut self, mv: &GameMove, libs: &Vec<(usize, usize)>) {
        self.group.insert(mv.pos);
        self.liberties.extend(libs);
        self.remove_liberty(mv.pos);
    }

    fn remove_liberty(&mut self, pos: (usize, usize)) {
        self.liberties.remove(&pos);
    }
}
