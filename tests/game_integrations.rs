use go::{game::*, stone::Stone, ui::*};
use std::io::{Cursor, Read, Write};

fn setup_game(input: &str) -> Game<TextUi<impl Read, impl Write>> {
    let reader = Cursor::new(String::from(input));
    let writer: Vec<u8> = vec![];
    let ui = TextUi::new(reader, writer);
    Game::new_game(9, 9, ui)
}

#[test]
fn make_move() {
    let mut game = setup_game("a1\n");
    game.update().unwrap();
    assert_eq!(Stone::Black, game.board.stone_at(0, 0));
}

#[test]
fn make_two_moves() {
    let mut game = setup_game("a1\nb2\n");
    game.update().unwrap();
    game.update().unwrap();
    assert_eq!(Stone::Black, game.board.stone_at(0, 0));
    assert_eq!(Stone::White, game.board.stone_at(1, 1));
}
