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
        for (idx, row) in self.state.iter().enumerate() {
            let icon = ((idx + 'A' as usize) as u8) as char;
            // unicode 2503 heavy box vertical
            let icon = icon.to_string() + "┃ ";
            ascii.push(icon.white());
            for stone in row {
                ascii.push(stone.unwrap().get_icon());
                // add whitespace for better looking board
                ascii.push(" ".white());
            }
            ascii.push("\n".white());
        }

        let mut row_len = 0;
        let mut iter = ascii.iter();
        while iter.next().unwrap() != &"\n".white() {
            row_len += 1;
        }
        // Add connector
        ascii.push(" ┗".white());
        for _ in 1..row_len {
            // add lines for columns
            ascii.push("━".white()); 
        }
        ascii.push("\n".white());
        ascii.push("   ".white());
        let mut idx = 1;
        for i in 1..row_len {
            if ascii[i] != " ".white(){
                ascii.push(idx.to_string().white());
                idx += 1;
            }
            else {
                ascii.push(" ".white());
            }
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
