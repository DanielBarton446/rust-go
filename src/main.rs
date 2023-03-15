use go::game::*;

fn main() {
    println!("Hello, world!");
    let mut game = Game::new_game(9, 9);
    game.start_game();
}
