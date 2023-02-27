use go::{game, stone::Stone};
use std::io::{self, Cursor};

#[test]
fn get_move() {
    let game = game::Game::new_game();
    let reader = Cursor::new(String::from("a1\n"));
    let mut writer: Vec<u8> = vec![];
    let res = game.get_move(reader, &mut writer);
    writer.iter().for_each(|b| print!("{}", *b as char));
    assert_eq!("A1", res.unwrap());
}

#[test]
fn make_move() {
    let mut game = game::Game::new_game();
    let reader = Cursor::new(String::from("A1\n"));
    let mut writer: Vec<u8> = vec![];
    game.make_move(reader, &mut writer).unwrap();
    writer.iter().for_each(|b| print!("{}", *b as char));
    assert_eq!(Stone::Black, game.board.stone_at(1, 0).unwrap());
}

#[test]
fn make_two_moves() {
    let mut game = game::Game::new_game();
    let reader_1 = Cursor::new(String::from("A1\n"));
    let reader_2 = Cursor::new(String::from("B1\n"));
    let mut writer: Vec<u8> = vec![];
    game.make_move(reader_1, &mut writer).unwrap();
    game.make_move(reader_2, &mut writer).unwrap();
    writer.iter().for_each(|b| print!("{}", *b as char));
    assert_eq!(Stone::Black, game.board.stone_at(1, 0).unwrap());
    assert_eq!(Stone::White, game.board.stone_at(1, 1).unwrap());
}
