//! This crate is an implementation of the game "Go", AKA
//! Weiqi, Baduk, Igo
//!
//! This crate features the following:
//! - TUI so you can play from your terminal
//! - Integration with OGS(in progress)

mod game_logic;
mod ui;

pub(crate) use game_logic::union_find;

pub use game_logic::board::Board;
pub use game_logic::game;
pub use game_logic::stone::Stone;
pub use ui::*;
