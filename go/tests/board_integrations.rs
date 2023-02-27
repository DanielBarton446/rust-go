use colored::Colorize;
use go::board;
use go::stone::Stone;

#[test]
fn new_board() {
    let board = board::Board::new(5, 5);
    let mut expected = Vec::new();
    for _ in 0..board.width {
        for _ in 0..board.height {
            expected.push(".".yellow());
            // add whitespace for better looking board
            expected.push(" ".yellow());
        }
        expected.push("\n".white());
    }

    // print!("{}", board);
    // assert_eq!(expected, board.to_ascii());
}

#[test]
fn new_board_add_stone() {
    let mut board = board::Board::new(9, 9);
    let black = Stone::Black;
    let white = Stone::White;
    board.place_stone(1, 0, black);
    board.place_stone(1, 1, white);
    board.place_stone(0, 1, black);
    board.place_stone(2, 1, black);
    board.place_stone(1, 2, black);
    println!("{board}");
    // assert_eq!(stone, board.stone_at(1, 1).unwrap())
}
