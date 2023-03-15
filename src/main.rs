use go::game::*;
use anyhow::Result;
use go::ui::TextUi;
use std::io;


fn main() -> Result<()> {
    let mut game = Game::new_game(9, 9, TextUi::new(io::stdin(), io::stdout()));
    game.start_game()?;
    Ok(())
}
