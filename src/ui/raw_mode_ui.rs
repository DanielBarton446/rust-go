#![allow(unused)]
use super::*;

use crate::game_logic::board::Board;
use crate::game_logic::stone::Icon;

use std::io::*;

use anyhow::{Context, Error, Result};
use colored::Colorize;
use crossterm::{event::*, style::*, terminal::*, *};

pub struct RawModeUi;

impl RawModeUi {
    pub fn new() -> Self {
        execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All));
        Self
    }
}

impl Default for RawModeUi {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RawModeUi {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen);
    }
}

impl UserInterface for RawModeUi {
    fn input(&mut self) -> Result<UserAction> {
        write!(stdout(), "Enter a move, or quit (q)")?;
        stdout().flush()?;

        let mut inp = String::new();
        stdin()
            .read_line(&mut inp)
            .with_context(|| "Failed to read input")?;

        if inp.trim() == "q" {
            return Ok(UserAction::Quit);
        }
        let mv = parse_move_position(&inp)?;
        Ok(UserAction::Move(mv.0, mv.1))
    }

    fn view(&mut self, board: &Board) -> Result<()> {
        queue!(stdout(), Clear(ClearType::All));
        writeln!(stdout(), "{board}")?;
        stdout().flush()?;
        Ok(())
    }
}
