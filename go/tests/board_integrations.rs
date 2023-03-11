use colored::Colorize;
use go::board;
use go::game_move::GameMove;
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
    board.update_board_state(&GameMove::new(black, (1,0), 0));
    board.update_board_state(&GameMove::new(white, (0,1), 0));
    board.update_board_state(&GameMove::new(black, (2,1), 0));
    board.update_board_state(&GameMove::new(white, (1,2), 0));
    println!("{board}");
    // assert_eq!(stone, board.stone_at(1, 1).unwrap())
}
