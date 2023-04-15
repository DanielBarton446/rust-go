use anyhow::Result;

use crate::game_logic::{board::*, game_move::GameMove, stone::Stone};
use crate::ui::*;

#[derive(Debug)]
pub struct Game<UI> {
    pub board: Board,
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
        Game {
            board,
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

    fn make_move(&mut self, row: usize, col: usize) -> Result<()> {
        let stn = if self.turn {
            Stone::Black
        } else {
            Stone::White
        };
        self.move_number += 1;
        let mv = GameMove::new(stn, (row, col), self.move_number);
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
}
