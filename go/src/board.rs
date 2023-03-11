use crate::{stone::*, game_move::GameMove};
use colored::{ColoredString, Colorize};
use std::fmt::Display;
use crate::chain::Chain;

pub struct Board {
    state: Vec<Vec<Stone>>,
    chains: Vec<Chain>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    /// * `width` - width of board
    /// * `height` - height of board
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            state: vec![vec![Stone::Empty; width]; height],
            chains: Vec::new(),
            width,
            height,
        }
    }

    pub fn get_liberties_of_pos(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        // let mut liberties: Vec<(usize, usize)> = Vec::new();
        // for i in self.height.saturating_sub(1)..=(pos.1+1).min(self.height) {
        //     for j in self.width.saturating_sub(1)..=(pos.0+1).min(self.width) {
        //         if self.state[i][j].unwrap() == Stone::Empty {
        //             // TODO: ignore diagonals
        //             liberties.push((j,i))
        //         }
        //     }
        // }
        let mut liberties: Vec<(usize, usize)> = Vec::new();
        if pos.0 > 0
            && self.in_bounds((pos.0 - 1, pos.1))
            && self.state[pos.0 - 1][pos.1] == Stone::Empty
        {
            liberties.push((pos.0 - 1, pos.1))
        }
        if pos.1 > 0
            && self.in_bounds((pos.0, pos.1 - 1))
            && self.state[pos.0][pos.1 - 1] == Stone::Empty
        {
            liberties.push((pos.0, pos.1 - 1))
        }
        if self.in_bounds((pos.0 + 1, pos.1))
            && self.state[pos.0 + 1][pos.1] == Stone::Empty
        {
            liberties.push((pos.0 + 1, pos.1))
        }
        if self.in_bounds((pos.0, pos.1 + 1))
            && self.state[pos.0][pos.1 + 1] == Stone::Empty
        {
            liberties.push((pos.0, pos.1 + 1))
        }

        liberties
    }

    pub fn in_bounds(&self, mv: (usize, usize)) -> bool {
        // usize representing board space, so no need to check >= 0
        mv.0 < self.width - 1 && mv.1 < self.height - 1
    }

    pub fn update_board_state(&mut self, mv: &GameMove) {
        self.place_stone(mv);
        // This really sucks to need to do
        for c in &self.chains {
            if c.is_dead_chain() {
                for pos in &c.group {
                    self.state[pos.1][pos.0] = Stone::Empty;
                } 
            }
        }
    }

    fn place_stone(&mut self, mv: &GameMove) {
        self.state[mv.pos.1][mv.pos.0] = mv.stone;
        let libs = self.get_liberties_of_pos(mv.pos);
        let mut joined_existing_chain = false;
        for c in &mut self.chains {
            if c.liberties.contains(&mv.pos){
                c.place_stone_and_liberties(mv, &libs);
                joined_existing_chain = true;
            }
        }
        if !joined_existing_chain {
            let c = Chain::new(mv, &libs);
            self.chains.push(c);
        }
    }

    pub fn stone_at(&self, x: usize, y: usize) -> Stone {
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
                        ascii.push(stone.get_icon());
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
