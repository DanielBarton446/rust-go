use go::{game, stone::Stone, ui};
use std::io::Cursor;

#[test]
fn get_move() {
    let reader = Cursor::new(String::from("a1\n"));
    let mut writer: Vec<u8> = vec![];
    let res = ui::get_move(reader, &mut writer);
    writer.iter().for_each(|b| print!("{}", *b as char));
    assert_eq!((0, 0), res.unwrap());
}

#[test]
fn make_move() {
    let mut game = game::Game::new_game(9,9);
    let reader = Cursor::new(String::from("A1\n"));
    let mut writer: Vec<u8> = vec![];
    game.make_move(reader, &mut writer).unwrap();
    writer.iter().for_each(|b| print!("{}", *b as char));
    assert_eq!(Stone::Black, game.board.stone_at(0, 0));
}

#[test]
fn make_two_moves() {
    let mut game = game::Game::new_game(9,9);
    let reader_1 = Cursor::new(String::from("A1\n"));
    let reader_2 = Cursor::new(String::from("B2\n"));
    let mut writer: Vec<u8> = vec![];
    game.make_move(reader_1, &mut writer).unwrap();
    game.make_move(reader_2, &mut writer).unwrap();
    writer.iter().for_each(|b| print!("{}", *b as char));
    assert_eq!(Stone::Black, game.board.stone_at(0, 0));
    assert_eq!(Stone::White, game.board.stone_at(1, 1));
}
