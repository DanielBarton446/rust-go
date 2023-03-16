use anyhow::Result;
use go::game::*;
use go::ui::RawModeUi;

fn main() -> Result<()> {
    //let mut game = Game::new_game(9, 9, TextUi::new(io::stdin(), io::stdout()));
    let mut game = Game::new_game(9, 9, RawModeUi::new());
    game.start_game()?;
    Ok(())
}
