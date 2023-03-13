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
    let parts = mv.split_at(1);

    // Error if not alphabetic
    if !parts.0.chars().all(char::is_alphabetic) {
        bail!("Non-alphabetical first coordinate");
    }

    // Error if not usize
    if !parts.1.chars().all(char::is_numeric) {
        bail!("Non-digit second coordinate");
    }

    // shouldn't unwrap here
    let row = parts.0.chars().next().with_context(|| "Input was empty")? as usize - 'A' as usize;
    let col: usize = parts.1.parse().with_context(|| "Input was only 1 character")?;
    Ok((row, col - 1))
}
