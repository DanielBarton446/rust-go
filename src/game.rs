use std::{io, usize};

use anyhow::{Context, Result};

use crate::{board::*, game_move::GameMove, stone::Stone, ui::get_move};

pub struct Game {
    pub board: Board,
    // players: TODO
    // timer: TODO
    // board_history: TODO
    pub turn: bool,
    pub move_number: usize,
}

impl Game {
    pub fn new_game(width: usize, height: usize) -> Self {
        let board = Board::new(width, height);
        Game {
            board,
            turn: true,
            move_number: 0,
        }
    }

    pub fn start_game(&mut self) {
        loop {
            self.print_board();
            self.make_move(io::BufReader::new(io::stdin()), &mut io::stdout())
                .unwrap();
        }
    }

    pub fn print_board(&self) {
        println!("{}", self.board);
    }

    pub fn make_move<R, W>(&mut self, reader: R, writer: &mut W) -> Result<()>
    where
        R: io::BufRead,
        W: io::Write,
    {
        let (row, col) = get_move(reader, writer).with_context(|| "Failed to get move")?;
        let mut stn = Stone::Empty;
        if self.turn {
            stn = Stone::Black;
        } else {
            stn = Stone::White;
        }
        self.move_number += 1;
        let mv = GameMove::new(stn, (row, col), self.move_number);
        self.board.update_board_state(&mv);
        self.turn = !self.turn;
        Ok(())
    }
}
