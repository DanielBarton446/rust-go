//! This module represents the game state and is where you can start to play the game.
use anyhow::Result;

use crate::game_logic::{board::*, game_move::GameMove, stone::Stone};
use crate::ui::*;
use crate::union_find::UnionFind;

#[derive(Debug)]
pub struct Game<UI> {
    pub board: Board,
    stone_groups: UnionFind,
    // players: TODO
    // timer: TODO
    // board_history: TODO
    pub(crate) turn: bool,
    pub(crate) move_number: usize,
    game_over: bool,
    ui: UI,
}

impl<UI: UserInterface> Game<UI> {
    /// Create a new game, specifying the width, height, and UI type
    /// of the board that you are going to use.
    ///
    /// For example:
    /// ```rust
    /// use crate::go::game::*;
    /// use go::RawModeUi;
    /// let mut game: Game<RawModeUi> = Game::new_game(3, 3, Default::default());
    /// ```
    /// This will create a game object with the default option, which
    /// in our case is the TUI UI.
    pub fn new_game(width: usize, height: usize, ui: UI) -> Self {
        let board = Board::new(width, height);
        let stone_groups = UnionFind::new(width * height);
        Game {
            board,
            stone_groups,
            turn: true,
            move_number: 0,
            game_over: false,
            ui,
        }
    }

    /// Start the game associated with this object.
    /// Note: This assumes the game will be played like any standard
    /// game would be played.
    pub fn start_game(&mut self) -> Result<()> {
        loop {
            if self.game_over {
                return Ok(());
            }
            self.ui.view(&self.board)?;
            self.update()?;
        }
    }

    /// This function updates the game based on User actions.
    /// Abstractoin from the UI trait. With OGS(todo), Opponent actions will be listened for
    fn update(&mut self) -> Result<()> {
        match self.ui.input()? {
            UserAction::Move(row, col) => self.make_move(row, col),
            UserAction::Quit => {
                self.game_over = true;
                Ok(())
            }
            UserAction::Noop => Ok(()),
        }
    }

    /// Strictly checkes that you are not placing a stone on an existing stone
    /// This should also check that row and column are within bounds
    fn is_valid_move(&mut self, row: usize, col: usize) -> bool {
        self.board.state[row][col] == Stone::Empty
    }

    /// This function checks the local "neighbors" and dispatches events
    /// based on different cases.
    ///
    /// Cases:
    /// - If there is a neighboring stone of the same color, we need to
    /// combine the two stones since they are a connected "chain".
    /// - If there is a neighboring stone of the opposing color, we need
    /// to remove the placed stone's position from the opposing stones'
    /// liberties list. Thereby needing to check for a capture.
    ///
    fn update_board(&mut self, mv: (usize, usize), stone: Stone) {
        let (row, col) = mv;
        let manhattan_adjacencies = [
            (row.checked_sub(1), col.checked_mul(1)),
            (row.checked_add(1), col.checked_mul(1)),
            (row.checked_mul(1), col.checked_sub(1)),
            (row.checked_mul(1), col.checked_add(1)),
        ];
        for (r, c) in manhattan_adjacencies {
            if let (Some(adj_row), Some(adj_col)) = (r, c) {
                if adj_row >= self.board.width || adj_col >= self.board.height {
                    continue; // skip if out of bounds
                }

                let move_index = self.board.index_of_pos(mv);
                let adjacent_index = self.board.index_of_pos((adj_row, adj_col));
                let existing_stone = self.board.state[adj_row][adj_col];

                if existing_stone == stone {
                    // union update stuff
                    self.stone_groups.union(move_index, adjacent_index);
                } else if existing_stone == stone.get_opponent().unwrap() {
                    // remove current move position from this adjacent stones representative chain
                    self.stone_groups
                        .remove_liberty_from_chain(adjacent_index, move_index);

                    dbg!("\nHERE\n{}", self.stone_groups.no_liberties(adjacent_index));
                    if self.stone_groups.no_liberties(adjacent_index) {
                        // stone here is dead. Update board
                        // TODO: need to update a prisoners list or something for captures
                        self.board.state[adj_row][adj_col] = Stone::Empty;
                    }
                }
            }
        }
    }

    /// This is a helper function that is in charge of updating the game
    /// based on a move given by a player.
    fn make_move(&mut self, row: usize, col: usize) -> Result<()> {
        if !self.is_valid_move(row, col) {
            return Ok(());
        }
        let stn = if self.turn {
            Stone::Black
        } else {
            Stone::White
        };
        self.move_number += 1;
        let mv = GameMove::new(stn, (row, col), self.move_number);
        self.create_libs(mv.pos);
        self.update_board(mv.pos, mv.stone);
        self.board.place_stone(&mv);
        self.turn = !self.turn;
        Ok(())
    }

    /// This is a helper function for the self.make_move function to initialize
    /// the liberties for the stone being placed.
    ///
    /// We can assume that the move is valid due to make_move checking for validity
    fn create_libs(&mut self, pos: (usize, usize)) {
        let (row, col) = pos;
        let manhattan_adjacencies = [
            (row.checked_sub(1), col.checked_mul(1)),
            (row.checked_add(1), col.checked_mul(1)),
            (row.checked_mul(1), col.checked_sub(1)),
            (row.checked_mul(1), col.checked_add(1)),
        ];

        let mut libs: Vec<usize> = Vec::with_capacity(4);

        for (r, c) in manhattan_adjacencies {
            if let (Some(adj_row), Some(adj_col)) = (r, c) {
                if adj_row >= self.board.width || adj_col >= self.board.height {
                    continue; // skip if out of bounds
                }
                libs.push(self.board.index_of_pos((adj_row, adj_col)));
            }
        }
        self.stone_groups
            .initialize_liberties_of_pos(self.board.index_of_pos(pos), libs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, io::*};

    fn setup_game(input: &str) -> Game<TextUi<impl Read, impl Write>> {
        let reader = Cursor::new(String::from(input));
        let writer: Vec<u8> = vec![];
        let ui = TextUi::new(reader, writer);
        Game::new_game(9, 9, ui)
    }

    #[test]
    fn make_move() {
        let mut game = setup_game("a1\n");
        game.update().unwrap();
        assert_eq!(Stone::Black, game.board.stone_at(0, 0));
    }

    #[test]
    fn make_two_moves() {
        let mut game = setup_game("a1\nb2\n");
        game.update().unwrap();
        game.update().unwrap();
        assert_eq!(Stone::Black, game.board.stone_at(0, 0));
        assert_eq!(Stone::White, game.board.stone_at(1, 1));
    }

    #[test]
    fn make_two_connecting_moves() {
        // let mut game = setup_game("a1\nb2\n");
        let mut game: Game<RawModeUi> = Game::new_game(5, 5, Default::default());
        game.make_move(0, 0).unwrap();
        game.turn = !game.turn;
        game.make_move(0, 1).unwrap();
        assert_eq!(Stone::Black, game.board.stone_at(0, 0));
        assert_eq!(Stone::Black, game.board.stone_at(0, 1));
        assert!(game.stone_groups.connected(
            game.board.index_of_pos((0, 0)),
            game.board.index_of_pos((0, 1)),
        ));
    }

    #[test]
    fn merge_two_chains_together() {
        // let mut game = setup_game("a1\nb2\n");
        let mut game: Game<RawModeUi> = Game::new_game(5, 5, Default::default());
        game.make_move(0, 0).unwrap();
        game.turn = !game.turn;
        game.make_move(0, 2).unwrap();
        game.turn = !game.turn;
        game.make_move(0, 1).unwrap();
        assert_eq!(Stone::Black, game.board.stone_at(0, 0));
        assert_eq!(Stone::Black, game.board.stone_at(0, 1));
        assert_eq!(Stone::Black, game.board.stone_at(0, 2));
        assert!(game.stone_groups.connected(
            game.board.index_of_pos((0, 0)),
            game.board.index_of_pos((0, 1)),
        ));
        assert!(game.stone_groups.connected(
            game.board.index_of_pos((0, 0)),
            game.board.index_of_pos((0, 2)),
        ));
        assert!(game.stone_groups.connected(
            game.board.index_of_pos((0, 1)),
            game.board.index_of_pos((0, 2)),
        ));
    }

    #[test]
    fn dead_corner_stone() {
        let mut game: Game<RawModeUi> = Game::new_game(3, 3, Default::default());
        game.make_move(0, 0).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(2, 2).unwrap();
        game.make_move(1, 0).unwrap();
        // dbg!(&board.chains.len());
        // dbg!(&game.board);
        print!("{}", &game.board);
        print!("{:?}", &game.stone_groups);
        // assert_eq!(Stone::Empty, game.board.stone_at(0, 0));
        let expected_board_state = vec![
            vec![Stone::Empty, Stone::White, Stone::Empty],
            vec![Stone::White, Stone::Empty, Stone::Empty],
            vec![Stone::Empty, Stone::Empty, Stone::Black],
        ];
        assert_eq!(game.board.state, expected_board_state);
    }

    #[test]
    fn correct_liberty_assignments() {
        let mut game: Game<RawModeUi> = Game::new_game(3, 3, Default::default());
        game.make_move(0, 0).unwrap();
        game.make_move(1, 1).unwrap();
        game.make_move(2, 2).unwrap();

        // beginning of array
        let expected_libs = HashSet::from_iter(vec![1, 3]);
        assert_eq!(game.stone_groups.liberties[0], expected_libs);

        // middle of board
        let expected_libs = HashSet::from_iter(vec![1, 3, 5, 7]);
        assert_eq!(game.stone_groups.liberties[4], expected_libs);

        // end of array
        let expected_libs = HashSet::from_iter(vec![5, 7]);
        assert_eq!(game.stone_groups.liberties[8], expected_libs);
    }
    //
    // #[test]
    // fn dead_side_stones() {
    //     let mut board = Board::new(9, 9);
    //     let black = Stone::Black;
    //     let white = Stone::White;
    //     board.update_board_state(&GameMove::new(black, (0, 1), 0));
    //     board.update_board_state(&GameMove::new(black, (0, 2), 0));
    //     board.update_board_state(&GameMove::new(white, (0, 0), 0));
    //     board.update_board_state(&GameMove::new(white, (1, 1), 0));
    //     board.update_board_state(&GameMove::new(white, (1, 2), 0));
    //     board.update_board_state(&GameMove::new(white, (0, 3), 0));
    //     assert_eq!(Stone::Empty, board.stone_at(0, 1));
    //     assert_eq!(Stone::Empty, board.stone_at(0, 2));
    //     assert_eq!(white, board.stone_at(0, 0));
    //     assert_eq!(white, board.stone_at(1, 1));
    //     assert_eq!(white, board.stone_at(1, 2));
    //     assert_eq!(white, board.stone_at(0, 3));
    // }
    //
    // #[test]
    // fn dead_center_stone() {
    //     let mut board = Board::new(9, 9);
    //     let black = Stone::Black;
    //     let white = Stone::White;
    //     board.update_board_state(&GameMove::new(black, (4, 4), 0));
    //     board.update_board_state(&GameMove::new(white, (3, 4), 0));
    //     board.update_board_state(&GameMove::new(white, (5, 4), 0));
    //     board.update_board_state(&GameMove::new(white, (4, 3), 0));
    //     board.update_board_state(&GameMove::new(white, (4, 5), 0));
    //     assert_eq!(Stone::Empty, board.stone_at(4, 4));
    //     assert_eq!(white, board.stone_at(3, 4));
    //     assert_eq!(white, board.stone_at(5, 4));
    //     assert_eq!(white, board.stone_at(4, 3));
    //     assert_eq!(white, board.stone_at(4, 5));
    // }
}
