use anyhow::Result;
use go::{game::Game, *};

fn main() -> Result<()> {
    let mut game: Game<RawModeUi> = Game::new_game(9, 9, Default::default());
    game.start_game()?;
    Ok(())
}
