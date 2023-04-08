use crate::chain::Chain;
use crate::{game_move::GameMove, stone::*};
use colored::{ColoredString, Colorize};
use std::fmt::Display;

#[derive(Debug)]
pub struct Board {
    pub(crate) state: Vec<Vec<Stone>>,
    chains: Vec<Chain>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Board {
    /// * `width` - width of board
    /// * `height` - height of board
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Board {
            state: vec![vec![Stone::Empty; width]; height],
            chains: Vec::new(),
            width,
            height,
        }
    }

    pub fn get_state(&self) -> Vec<&[Stone]> {
        self.state.iter().map(|v| v.as_slice()).collect()
    }

    /// Returns the chain's adjacent up/down/left/right squares that are empty
    pub(crate) fn get_liberties_of_pos(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut liberties: Vec<(usize, usize)> = Vec::new();
        let row = pos.0;
        let col = pos.1;

        // left
        if col > 0 && self.in_bounds(row, col - 1) && self.state[row][col - 1] == Stone::Empty {
            liberties.push((row, col - 1))
        }

        // right
        if self.in_bounds(row, col + 1) && self.state[row][col + 1] == Stone::Empty {
            liberties.push((row, col + 1))
        }

        // up
        if row > 0 && self.in_bounds(row - 1, col) && self.state[row - 1][col] == Stone::Empty {
            liberties.push((row - 1, col))
        }

        // down
        if self.in_bounds(row + 1, col) && self.state[row + 1][col] == Stone::Empty {
            liberties.push((row + 1, col))
        }

        liberties
    }

    pub(crate) fn in_bounds(&self, row: usize, col: usize) -> bool {
        // usize representing board space, so no need to check >= 0
        col < self.width && row < self.height
    }

    pub(crate) fn update_board_state(&mut self, mv: &GameMove) {
        self.place_stone(mv);
        // This really sucks to need to do
        for c in &self.chains {
            if c.is_dead_chain() {
                for pos in &c.group {
                    self.state[pos.0][pos.1] = Stone::Empty;
                }
            }
        }
    }

    fn place_stone(&mut self, mv: &GameMove) {
        self.state[mv.pos.0][mv.pos.1] = mv.stone;
        let libs = self.get_liberties_of_pos(mv.pos);
        let mut joined_existing_chain = false;
        for c in &mut self.chains {
            if c.liberties.contains(&mv.pos) {
                c.place_stone_and_update_liberties(mv, &libs);
                if c.color == mv.stone {
                    // only if joining ex
                    joined_existing_chain = true;
                }
            }
        }
        // create new chain if this stone has no ally neighbors
        if !joined_existing_chain {
            let c = Chain::new(mv, &libs);
            self.chains.push(c);
        }

        self.merge_chains_if_needed(mv);

    }

    fn merge_chains_if_needed(&mut self, mv: &GameMove) {
        let mut ally_adjacent_chains: Vec<Chain> = Vec::new();
        let mut i = 0;
        while i < self.chains.len() {
            // check chains that have stone added to group
            if self.chains[i].group.contains(&mv.pos) {
                // remove chain from board and handle merging
                ally_adjacent_chains.push(self.chains.remove(i));
            }
            else {
                i += 1;
            }
        }

        if !ally_adjacent_chains.is_empty(){
            let mut head = ally_adjacent_chains.remove(0);
            while !ally_adjacent_chains.is_empty(){
                head.extend_chain(ally_adjacent_chains.remove(0)); // this should be handled
            }
            self.chains.push(head);
        }

    }

    #[cfg(test)]
    pub(crate) fn stone_at(&self, row: usize, col: usize) -> Stone {
        self.state[row][col]
    }

    // This function is awful, primarily because of the fact that
    // we have a Vec of strings. Not a vec of chars. Make sizing
    // nightmarish.
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for msg in &self.to_ascii() {
            write!(f, "{}", msg)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::format;

    use super::*;

    #[test]
    fn new_board_add_stone() {
        let mut board = Board::new(9, 9);
        let black = Stone::Black;
        let white = Stone::White;
        board.update_board_state(&GameMove::new(black, (1, 0), 0));
        board.update_board_state(&GameMove::new(white, (0, 1), 0));
        board.update_board_state(&GameMove::new(black, (2, 1), 0));
        board.update_board_state(&GameMove::new(white, (1, 2), 0));
        assert_eq!(black, board.stone_at(1, 0));
        assert_eq!(black, board.stone_at(2, 1));
        assert_eq!(white, board.stone_at(0, 1));
        assert_eq!(white, board.stone_at(1, 2));
    }

    #[test]
    fn dead_corner_stone() {
        let mut board = Board::new(9, 9);
        let black = Stone::Black;
        let white = Stone::White;
        board.update_board_state(&GameMove::new(black, (0, 0), 0));
        board.update_board_state(&GameMove::new(white, (0, 1), 1));
        board.update_board_state(&GameMove::new(white, (1, 0), 2));
        println!("{}", &board);
        dbg!(&board);
        // dbg!(&board.chains.len());
        assert_eq!(Stone::Empty, board.stone_at(0, 0));
        assert_eq!(white, board.stone_at(0, 1));
        assert_eq!(white, board.stone_at(1, 0));
    }

    #[test]
    fn dead_side_stones() {
        let mut board = Board::new(9, 9);
        let black = Stone::Black;
        let white = Stone::White;
        board.update_board_state(&GameMove::new(black, (0, 1), 0));
        board.update_board_state(&GameMove::new(black, (0, 2), 0));
        board.update_board_state(&GameMove::new(white, (0, 0), 0));
        board.update_board_state(&GameMove::new(white, (1, 1), 0));
        board.update_board_state(&GameMove::new(white, (1, 2), 0));
        board.update_board_state(&GameMove::new(white, (0, 3), 0));
        assert_eq!(Stone::Empty, board.stone_at(0, 1));
        assert_eq!(Stone::Empty, board.stone_at(0, 2));
        assert_eq!(white, board.stone_at(0, 0));
        assert_eq!(white, board.stone_at(1, 1));
        assert_eq!(white, board.stone_at(1, 2));
        assert_eq!(white, board.stone_at(0, 3));
    }

    #[test]
    fn dead_center_stone() {
        let mut board = Board::new(9, 9);
        let black = Stone::Black;
        let white = Stone::White;
        board.update_board_state(&GameMove::new(black, (4, 4), 0));
        board.update_board_state(&GameMove::new(white, (3, 4), 0));
        board.update_board_state(&GameMove::new(white, (5, 4), 0));
        board.update_board_state(&GameMove::new(white, (4, 3), 0));
        board.update_board_state(&GameMove::new(white, (4, 5), 0));
        assert_eq!(Stone::Empty, board.stone_at(4, 4));
        assert_eq!(white, board.stone_at(3, 4));
        assert_eq!(white, board.stone_at(5, 4));
        assert_eq!(white, board.stone_at(4, 3));
        assert_eq!(white, board.stone_at(4, 5));
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
        board.update_board_state(&GameMove::new(Stone::Black, (2, 2), 0));
        assert_eq!(board.get_state()[2][2], Stone::Black);
    }

    #[test]
    fn get_liberties_of_corner() {
        let board = Board::new(3, 3);
        dbg!(&board);
        let libs = board.get_liberties_of_pos((2, 2));
        assert_eq!(libs, vec![(2, 1), (1, 2),])
    }

    #[test]
    fn merge_two_chains_test(){
        let mut board = Board::new(9, 9);
        let black = Stone::Black;
        board.update_board_state(&GameMove::new(black, (0, 0), 0));
        board.update_board_state(&GameMove::new(black, (0, 2), 0));
        board.update_board_state(&GameMove::new(black, (0, 1), 0));
        dbg!(&board.chains);
        assert_eq!(board.chains.len(), 1);
        assert_eq!(board.chains[0].group.len(), 3);
        assert!(board.chains[0].group.contains(&(0, 0)));
        assert!(board.chains[0].group.contains(&(0, 1)));
        assert!(board.chains[0].group.contains(&(0, 2)));
    }

    #[test]
    fn merge_three_chains_test(){
        let mut board = Board::new(9, 9);
        let black = Stone::Black;
        board.update_board_state(&GameMove::new(black, (0, 0), 0));
        board.update_board_state(&GameMove::new(black, (0, 2), 0));
        board.update_board_state(&GameMove::new(black, (1, 1), 0));
        board.update_board_state(&GameMove::new(black, (0, 1), 0));
        dbg!(&board.chains);
        assert_eq!(board.chains.len(), 1);
        assert_eq!(board.chains[0].group.len(), 4);
        assert!(board.chains[0].group.contains(&(0, 0)));
        assert!(board.chains[0].group.contains(&(0, 1)));
        assert!(board.chains[0].group.contains(&(1, 1)));
        assert!(board.chains[0].group.contains(&(0, 2)));
    }


    #[test]
    fn merge_four_chains_test(){
        let mut board = Board::new(9, 9);
        let black = Stone::Black;
        board.update_board_state(&GameMove::new(black, (1, 0), 0));
        board.update_board_state(&GameMove::new(black, (0, 1), 0));
        board.update_board_state(&GameMove::new(black, (2, 1), 0));
        board.update_board_state(&GameMove::new(black, (1, 2), 0));
        board.update_board_state(&GameMove::new(black, (1, 1), 0));
        dbg!(&board.chains);
        assert_eq!(board.chains.len(), 1);
        assert_eq!(board.chains[0].group.len(), 5);
        assert!(board.chains[0].group.contains(&(1, 0)));
        assert!(board.chains[0].group.contains(&(0, 1)));
        assert!(board.chains[0].group.contains(&(2, 1)));
        assert!(board.chains[0].group.contains(&(1, 2)));
        assert!(board.chains[0].group.contains(&(1, 1)));
    }
}
