//! The ui module should contain all user interaction code. This allows your other code to focus on
//! game logic rather than handling inputs/outputs.

mod text_ui;
pub use text_ui::TextUi;

use crate::board::Board;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum UserAction {
    Move(usize, usize),
    Quit,
}

pub trait UserInterface {
    /// User input, which will be passed to the controller
    fn input(&mut self) -> Result<UserAction>;

    /// View the model
    fn view(&mut self, board: &Board) -> Result<()>;
}
