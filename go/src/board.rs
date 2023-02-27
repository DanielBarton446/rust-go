use crate::stone::*;
use colored::{ColoredString, Colorize};
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

    // This function is awful, primarily because of the fact that
    // we have a Vec of strings. Not a vec of chars. Make sizing
    // nightmarish.
    pub fn to_ascii(&self) -> Vec<colored::ColoredString> {
        let mut ascii: Vec<colored::ColoredString> = Vec::new();
        // legend_max_char_width examples:
        // '5' is 1 char long
        // '11' is 2 char long
        // Each of these need different padding size
        let legend_max_char_width = self.width.to_string().len();
        let padding = (0..legend_max_char_width).map(|_| " ").collect::<String>();
        for (idx, row) in self.state.iter().enumerate() {
            for i in 0..padding.len() {
                if i % padding.len() == 0 {
                    let legend = ((idx + 'A' as usize) as u8) as char;
                    let legend = legend.to_string() + "┃" + &padding;
                    ascii.push(legend.white());
                    // push stones on row
                    for stone in row {
                        ascii.push(stone.unwrap().get_icon());
                        // add whitespace for better looking board
                        ascii.push(padding.white());
                    }
                    ascii.push("\n".white());
                } else {
                    ascii.push(" ┃".white());
                    ascii.push("\n".white());
                }
            }
        }

        // Add connector
        ascii.push(" ┗".white());
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
        let mut column_legend: Vec<ColoredString> = Vec::new();
        let mut chunk = ascii.iter();
        chunk.next(); // skip over the row legend markers (e.g. A|)
        let mut skip_count = 0;
        for val in chunk {
            if val.contains('\n') {
                break;
            }
            for char in val.chars() {
                // adjust for n char long legend (e.g. '15' is 2 char long)
                if skip_count > 0 {
                    skip_count -= 1;
                    continue;
                }
                if char != ' ' {
                    column_legend.push(idx.to_string().white());
                    idx += 1;
                    skip_count = idx.to_string().len() - 1;
                } else {
                    column_legend.push(" ".white());
                }
            }
        }

        ascii.append(&mut column_legend); // add column_legend markers
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
