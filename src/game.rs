use anyhow::Result;

use crate::{board::*, game_move::GameMove, stone::Stone, ui::*};

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
