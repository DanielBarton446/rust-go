use crate::stone::*;
use colored::Colorize;
use std::fmt::Display;

pub struct Board {
    state: Vec<Vec<Option<Stone>>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    /// * `width` - width of board
    /// * `height` - height of board
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            state: vec![vec![Some(Stone::Empty); width]; height],
            width,
            height,
        }
    }

    pub fn place_stone(&mut self, x: usize, y: usize, stone: Stone) {
        self.state[y][x] = Some(stone);
    }

    pub fn stone_at(&self, x: usize, y: usize) -> Option<Stone> {
        self.state[y][x]
    }

    pub fn to_ascii(&self) -> Vec<colored::ColoredString> {
        let mut ascii: Vec<colored::ColoredString> = Vec::new();
        for row in &self.state {
            for stone in row {
                ascii.push(stone.unwrap().get_icon());
                // add whitespace for better looking board
                ascii.push(" ".yellow());
            }
            ascii.push("\n".white());
        }
        ascii
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for msg in &self.to_ascii() {
            write!(f, "{}", msg)?
        }
        Ok(())
    }
}
