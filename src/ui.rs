//! The ui module should contain all user interaction code. This allows your other code to focus on
//! game logic rather than handling inputs/outputs.
use anyhow::{ Result, bail, Context };
use std::io;

/// Prompts the user for a new move
pub fn get_move<R, W>(mut reader: R, mut writer: W) -> Result<(usize, usize)>
where
    R: io::BufRead,
    W: io::Write,
{
    let mut resp = String::new();
    writer
        .write_all(b"Enter your move:\n")
        .with_context(|| "Failed to write")?;
    reader.read_line(&mut resp).with_context(|| "Failed to readline")?;
    // #[cfg(test)]
    // This should only run when `cargo test`, but for some reason doesn't work
    // writer.write_all(resp.as_bytes()).expect("Failed to write");

    parse_move_position(resp.trim().to_ascii_uppercase())
}

fn parse_move_position(mv: String) -> Result<(usize, usize)> {
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
}
