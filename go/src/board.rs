use crate::stone::*;
use colored::{Colorize, ColoredString};
use std::{fmt::Display, ops::Index};

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
        let padding = "  ";
        for (idx, row) in self.state.iter().enumerate() {
            for i in 0..padding.len() {
                if i % padding.len() == 0{
                    let legend = ((idx + 'A' as usize) as u8) as char;
                    // unicode 2503 heavy box vertical
                    let legend = "\n".to_string() + &legend.to_string() + "┃" + padding;
                    ascii.push(legend.white());
                    // push stones on row
                    for stone in row {
                        ascii.push(stone.unwrap().get_icon());
                        // add whitespace for better looking board
                        ascii.push(padding.white());
                    }
                }
                else {
                    ascii.push("\n".white());
                    ascii.push(" ┃".white()) ;
                }
            }

        }

        // Add connector
        ascii.push("\n ┗".white());
        // padding * number of stones, plus the 1 char for each stone
        let row_len = padding.len() * self.width + self.width;
        for _ in 0..row_len {
            // add lines for columns
            ascii.push("━".white()); 
        }

        // spacing before column legend
        ascii.push("\n".white());
        for _ in 0..(padding.len() + 2) {
            ascii.push(" ".white());
        }

        // column legend
        let mut idx = 1;
        // this is awful. How can we stop the need for cloning?
        let binding = ascii.clone(); 
        let mut chunk = binding.iter();
        chunk.next(); // skip over first newline
        let mut skip_count = 0;
        for val in chunk {
            if val.contains('\n') {
                break
            }
            for char in val.chars() {
                // adjust for 2 digit long legend
                if skip_count > 0 {
                    skip_count -= 1;
                    continue;
                }
                if char != ' ' {
                    ascii.push(idx.to_string().white());
                    idx += 1;
                    skip_count = idx.to_string().len() - 1;
                }
                else {
                    ascii.push(" ".white());
                }
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
