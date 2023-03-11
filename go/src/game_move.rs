use crate::stone::Stone;

pub struct GameMove {
    pub stone: Stone,
    pub pos: (usize, usize),
    pub move_number: usize,
}

impl GameMove {
    pub fn new(stone: Stone, pos: (usize, usize), move_number: usize) -> Self {
        GameMove {
            stone,
            pos,
            move_number,
        }
    }
}
