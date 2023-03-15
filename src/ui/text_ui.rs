use super::*;
use crate::board::Board;
use anyhow::{bail, Context, Result};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

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
            return Ok(UserAction::Quit)
        }
        let mv = parse_move_position(inp)?;
        Ok(UserAction::Move(mv.0, mv.1))
    }

    fn view(&mut self, board: &Board) -> Result<()> {
        write!(self.writer, "{}\n", board).with_context(|| "Failed to prompt user")
    }
}

fn parse_move_position(mv: String) -> Result<(usize, usize)> {
    let mv = mv.trim();
    if mv.len() != 2 {
        bail!("Move should be 2 characters");
    }
    let row = mv.as_bytes()[0].to_ascii_uppercase();
    let col = mv.as_bytes()[1];

    // Error if not alphabetic
    if !row.is_ascii_alphabetic() {
        bail!("Non-alphabetical row");
    }
    if !col.is_ascii_digit() {
        bail!("Non-digit column");
    }

    // shouldn't unwrap here
    let row = (row - b'A') as usize;
    let col: usize = (col - b'0') as usize - 1;
    Ok((row, col))
}

#[cfg(test)]
mod tests {
    use super::*;

    // parse_move tests
    #[test]
    fn parse_move_works_uppper_case() {
        let input = String::from("A1");
        let res = parse_move_position(input).unwrap();
        assert_eq!((0, 0), res);
    }

    #[test]
    fn parse_move_works_lower_case() {
        let input = String::from("a1");
        let res = parse_move_position(input).unwrap();
        assert_eq!((0, 0), res);
    }

    #[test]
    fn parse_move_solo_alpha_should_error() {
        let input = String::from("a");
        parse_move_position(input).unwrap_err();
    }

    #[test]
    fn parse_move_solo_digit_should_error() {
        let input = String::from("1");
        parse_move_position(input).unwrap_err();
    }

    #[test]
    fn parse_move_empty_should_error() {
        let input = String::from("");
        parse_move_position(input).unwrap_err();
    }

    #[test]
    fn parse_move_all_digits_should_error() {
        let input = String::from("11");
        parse_move_position(input).unwrap_err();
    }

    #[test]
    fn parse_move_all_alpha_should_error() {
        let input = String::from("aa");
        parse_move_position(input).unwrap_err();
    }

    #[test]
    fn parse_move_extra_char_should_error() {
        let input = String::from("a1b");
        parse_move_position(input).unwrap_err();
    }

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
