//! The ui module should contain all user interaction code. This allows your other code to focus on
//! game logic rather than handling inputs/outputs.

mod raw_mode_ui;
mod text_ui;
pub use raw_mode_ui::RawModeUi;
pub use text_ui::{StdTextUi, TextUi};

use crate::game_logic::board::Board;
use anyhow::{bail, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum UserAction {
    Move(usize, usize),
    Quit,
    Noop,
}

pub trait UserInterface {
    /// User input, which will be passed to the controller
    fn input(&mut self) -> Result<UserAction>;

    /// View the model
    fn view(&mut self, board: &Board) -> Result<()>;
}

fn parse_move_position(mv: &str) -> Result<(usize, usize)> {
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
        let res = parse_move_position("A1").unwrap();
        assert_eq!((0, 0), res);
    }

    #[test]
    fn parse_move_works_lower_case() {
        let res = parse_move_position("a1").unwrap();
        assert_eq!((0, 0), res);
    }

    #[test]
    fn parse_move_solo_alpha_should_error() {
        parse_move_position("a").unwrap_err();
    }

    #[test]
    fn parse_move_solo_digit_should_error() {
        parse_move_position("1").unwrap_err();
    }

    #[test]
    fn parse_move_empty_should_error() {
        parse_move_position("").unwrap_err();
    }

    #[test]
    fn parse_move_all_digits_should_error() {
        parse_move_position("11").unwrap_err();
    }

    #[test]
    fn parse_move_all_alpha_should_error() {
        parse_move_position("aa").unwrap_err();
    }

    #[test]
    fn parse_move_extra_char_should_error() {
        parse_move_position("a1b").unwrap_err();
    }
}
