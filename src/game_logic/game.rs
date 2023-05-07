use std::collections::HashSet;

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

    pub fn start_game(&mut self) -> Result<()> {
        loop {
            if self.game_over {
                return Ok(());
            }
            self.ui.view(&self.board)?;
            self.update()?;
        }
    }

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

    fn is_valid_move(&mut self, row: usize, col: usize) -> Result<()> {
        if self.board.state[row][col] != Stone::Empty {
            println!("TODO: send back a bloody error heck");
        }
        Ok(())
    }

    fn update_chains(&mut self, mv: (usize, usize), stone: Stone) {
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
                if self.board.state[adj_row][adj_col] == stone {
                    dbg!(move_index, adjacent_index);
                    self.stone_groups.union(move_index, adjacent_index);
                }
            }
        }
    }

    fn make_move(&mut self, row: usize, col: usize) -> Result<()> {
        self.is_valid_move(row, col)?;
        let stn = if self.turn {
            Stone::Black
        } else {
            Stone::White
        };
        self.move_number += 1;
        let mv = GameMove::new(stn, (row, col), self.move_number);
        self.update_chains(mv.pos, mv.stone);
        self.board.update_board_state(&mv);
        self.turn = !self.turn;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::*;

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
        if let Err(e) = game.make_move(0, 0) {
            panic!("{}", e)
        }
        game.turn = !game.turn;
        if let Err(e) = game.make_move(0, 1) {
            panic!("{}", e)
        }
        assert_eq!(Stone::Black, game.board.stone_at(0, 0));
        assert_eq!(Stone::Black, game.board.stone_at(0, 1));
        assert!(game.stone_groups.connected(
            game.board.index_of_pos((0, 0)),
            game.board.index_of_pos((0, 1)),
        ));
    }

    #[ignore]
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

    // #[test]
    // fn dead_corner_stone() {
    //     let mut board = Board::new(9, 9);
    //     let black = Stone::Black;
    //     let white = Stone::White;
    //     board.update_board_state(&GameMove::new(black, (0, 0), 0));
    //     board.update_board_state(&GameMove::new(white, (0, 1), 1));
    //     board.update_board_state(&GameMove::new(white, (1, 0), 2));
    //     println!("{}", &board);
    //     dbg!(&board);
    //     // dbg!(&board.chains.len());
    //     assert_eq!(Stone::Empty, board.stone_at(0, 0));
    //     assert_eq!(white, board.stone_at(0, 1));
    //     assert_eq!(white, board.stone_at(1, 0));
    // }
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
