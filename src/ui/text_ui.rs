use super::*;
use crate::board::Board;
use anyhow::{Context, Result};
use std::io::*;

/// Wrapper over TextUi to make initialization of the stdin/stdout text ui simpler
pub struct StdTextUi {
    ui: TextUi<Stdin, Stdout>,
}

impl StdTextUi {
    fn new() -> Self {
        Self {
            ui: TextUi::new(stdin(), stdout()),
        }
    }
}

impl Default for StdTextUi {
    fn default() -> Self {
        Self::new()
    }
}

impl UserInterface for StdTextUi {
    fn view(&mut self, board: &Board) -> Result<()> {
        self.ui.view(board)
    }
    fn input(&mut self) -> Result<UserAction> {
        self.ui.input()
    }
}

#[derive(Debug)]
pub struct TextUi<R: Read, W: Write> {
    reader: std::io::BufReader<R>,
    writer: std::io::BufWriter<W>,
}

impl<R: Read, W: Write> TextUi<R, W> {
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer: BufWriter::new(writer),
        }
    }
}

impl<R: Read, W: Write> UserInterface for TextUi<R, W> {
    fn input(&mut self) -> Result<UserAction> {
        write!(self.writer, "Enter a move, or quit (q)")?;
        self.writer.flush()?;

        let mut inp = String::new();
        self.reader
            .read_line(&mut inp)
            .with_context(|| "Failed to read input")?;

        if inp.trim() == "q" {
            return Ok(UserAction::Quit);
        }
        let mv = parse_move_position(&inp)?;
        Ok(UserAction::Move(mv.0, mv.1))
    }

    fn view(&mut self, board: &Board) -> Result<()> {
        writeln!(self.writer, "{}", board).with_context(|| "Failed to prompt user")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_move() {
        let reader = std::io::Cursor::new(String::from("a1\n"));
        let mut ui = TextUi::new(reader, vec![]);
        let action = ui.input().unwrap();
        assert_eq!(UserAction::Move(0, 0), action);
    }

    #[test]
    fn get_quit() {
        let reader = std::io::Cursor::new(String::from("q\n"));
        let mut ui = TextUi::new(reader, vec![]);
        let action = ui.input().unwrap();
        assert_eq!(UserAction::Quit, action);
    }
}
