use std::{io, usize};

use crate::{board::*, stone::Stone};

pub struct Game {
    pub board: Board,
    // players: TODO
    // timer: TODO
    // board_history: TODO
    pub turn: bool,
}

impl Game {
    pub fn new_game() -> Self {
        let board = Board::new(9, 9);
        Game { board, turn: true }
    }

    pub fn start_game(&mut self) {
        loop {
            self.make_move(io::BufReader::new(io::stdin()), &mut io::stdout())
                .unwrap();
        }
    }

    fn parse_move(mv: String) -> Result<(usize, usize), String> {
        let parts = mv.split_at(1);

        // Error if not alphabetic
        if !parts.0.chars().all(char::is_alphabetic) {
            return Err("Non-alphabetical first coordinate".to_string());
        }

        // Error if not usize
        if !parts.1.chars().all(char::is_numeric) {
            return Err("Non-digit second coordinate".to_string());
        }
        // shouldn't unwrap here
        let y = parts.0.chars().next().unwrap() as usize - 'A' as usize;
        let x: usize = parts.1.parse().unwrap();
        Ok((x, y))
    }

    pub fn make_move<R, W>(&mut self, reader: R, writer: &mut W) -> Result<(), String>
    where
        R: io::BufRead,
        W: io::Write,
    {
        let mv = self.get_move(reader, writer);
        match mv {
            Ok(mv) => {
                let (x, y) = self::Game::parse_move(mv)?;
                self.place_stone(x, y);
                self.turn = !self.turn;
                Ok(())
            }
            io::Result::Err(e) => Err(e.to_string()),
        }
    }

    pub fn get_move<R, W>(&self, mut reader: R, mut writer: W) -> io::Result<String>
    where
        R: io::BufRead,
        W: io::Write,
    {
        let mut resp = String::new();
        writer
            .write_all(b"Enter your move:\n")
            .expect("Failed to write");
        reader.read_line(&mut resp).expect("Failed to readline");
        // #[cfg(test)]
        // This should only run when `cargo test`, but for some reason doesn't work
        // writer.write_all(resp.as_bytes()).expect("Failed to write");

        Ok(resp.trim().to_ascii_uppercase())
    }

    fn place_stone(&mut self, x: usize, y: usize) {
        // This is bad to do in longterm, since something else should be
        // deciding what color stone is placed
        if self.turn {
            self.board.place_stone(x, y, Stone::Black);
        } else {
            self.board.place_stone(x, y, Stone::White);
        }
    }
}
