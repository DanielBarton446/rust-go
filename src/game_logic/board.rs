// File documentation

/// The purpose of this file is to define the `Board` struct, which is strictly used for representing the state of the game.
/// This file should not contain any game-specific logic or rules. It solely provides a data structure to represent the board.
///
/// The `Board` struct serves as a container for storing information about the current state of the game board.
/// It is responsible for maintaining the board's dimensions, tracking stone positions, and providing basic operations
/// for accessing and manipulating the board's state.
///
/// This file does not handle game-specific logic, such as capturing stones, determining liberties, or enforcing game rules.
/// Those responsibilities should be implemented in separate modules or components that interact with the `Board` struct.
/// This separation of concerns ensures a clear distinction between the game representation and the game rules and logic.
///
/// Please note that this file is intended only for representing the game board's state and should not include any gameplay logic.
/// Game-related operations and rules should be implemented elsewhere in the codebase.
use crate::game_logic::game_move::*;
use crate::game_logic::stone::*;
use colored::{ColoredString, Colorize};
use std::fmt::Display;

/// This is a struct that represents strictly the `board` state for the game.
///
/// This struct contains a 2d vector to represent the board, and helpful
/// fields like width and height.
#[derive(Debug)]
pub struct Board {
    pub(crate) state: Vec<Vec<Stone>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Board {
    /// * `width` - width of board
    /// * `height` - height of board
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Board {
            state: vec![vec![Stone::Empty; width]; height],
            width,
            height,
        }
    }

    /// this flattens the 2d state of the board to a single array
    pub fn get_state(&self) -> Vec<&[Stone]> {
        self.state.iter().map(|v| v.as_slice()).collect()
    }

    /// This gets the 1d index given the cartesian coordinates
    pub fn index_of_pos(&self, pos: (usize, usize)) -> usize {
        // TODO: sanity check on bounds

        pos.0 * self.width + pos.1
    }

    /// Simply place the stone onto the board
    pub(crate) fn place_stone(&mut self, mv: &GameMove) {
        // TODO: Sanity check for bounds just in case.
        let row = mv.pos.0;
        let col = mv.pos.1;
        self.state[row][col] = mv.stone;
    }

    #[cfg(test)]
    pub(crate) fn stone_at(&self, row: usize, col: usize) -> Stone {
        self.state[row][col]
    }

    /// This function is used to pretty print the board to the terminal,
    /// which is used for the RawModeUI when playing with the TUI.
    /// This function is awful, primarily because of the fact that
    /// we have a Vec of strings. Not a vec of chars. Make sizing
    /// nightmarish.
    pub(crate) fn to_ascii(&self) -> Vec<colored::ColoredString> {
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
    /// Used for pretty printing of the Board object
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for msg in &self.to_ascii() {
            write!(f, "{}", msg)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_board_add_stone() {
        let mut board = Board::new(9, 9);
        let black = Stone::Black;
        let white = Stone::White;
        board.place_stone(&GameMove::new(black, (1, 0), 0));
        board.place_stone(&GameMove::new(white, (0, 1), 0));
        board.place_stone(&GameMove::new(black, (2, 1), 0));
        board.place_stone(&GameMove::new(white, (1, 2), 0));
        assert_eq!(black, board.stone_at(1, 0));
        assert_eq!(black, board.stone_at(2, 1));
        assert_eq!(white, board.stone_at(0, 1));
        assert_eq!(white, board.stone_at(1, 2));
    }

    #[test]
    fn place_stone_in_corner() {
        let mut board = Board::new(3, 3);
        board.place_stone(&GameMove::new(Stone::Black, (2, 2), 0));
        assert_eq!(board.get_state()[2][2], Stone::Black);
    }

    #[test]
    fn update_board_with_corner_move() {
        let mut board = Board::new(3, 3);
        board.place_stone(&GameMove::new(Stone::Black, (2, 2), 0));
        assert_eq!(board.get_state()[2][2], Stone::Black);
    }
}
