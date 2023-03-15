use go::board;
use go::game_move::GameMove;
use go::stone::Stone;

#[test]
fn new_board_add_stone() {
    let mut board = board::Board::new(9, 9);
    let black = Stone::Black;
    let white = Stone::White;
    board.update_board_state(&GameMove::new(black, (1,0), 0));
    board.update_board_state(&GameMove::new(white, (0,1), 0));
    board.update_board_state(&GameMove::new(black, (2,1), 0));
    board.update_board_state(&GameMove::new(white, (1,2), 0));
    assert_eq!(black, board.stone_at(1, 0));
    assert_eq!(black, board.stone_at(2, 1));
    assert_eq!(white, board.stone_at(0, 1));
    assert_eq!(white, board.stone_at(1, 2));
}

#[test]
fn dead_corner_stone() {
    let mut board = board::Board::new(9, 9);
    let black = Stone::Black;
    let white = Stone::White;
    board.update_board_state(&GameMove::new(black, (0,0), 0));
    board.update_board_state(&GameMove::new(white, (0,1), 1));
    board.update_board_state(&GameMove::new(white, (1,0), 2));
    assert_eq!(Stone::Empty, board.stone_at(0,0));
    assert_eq!(white, board.stone_at(0,1));
    assert_eq!(white, board.stone_at(1,0));
}


#[test]
fn dead_side_stones() {
    let mut board = board::Board::new(9, 9);
    let black = Stone::Black;
    let white = Stone::White;
    board.update_board_state(&GameMove::new(black, (0,1), 0));
    board.update_board_state(&GameMove::new(black, (0,2), 0));
    board.update_board_state(&GameMove::new(white, (0,0), 0));
    board.update_board_state(&GameMove::new(white, (1,1), 0));
    board.update_board_state(&GameMove::new(white, (1,2), 0));
    board.update_board_state(&GameMove::new(white, (0,3), 0));
    assert_eq!(Stone::Empty, board.stone_at(0,1));
    assert_eq!(Stone::Empty, board.stone_at(0,2));
    assert_eq!(white, board.stone_at(0,0));
    assert_eq!(white, board.stone_at(1,1));
    assert_eq!(white, board.stone_at(1,2));
    assert_eq!(white, board.stone_at(0,3));
}

#[test]
fn dead_center_stone() {
    let mut board = board::Board::new(9, 9);
    let black = Stone::Black;
    let white = Stone::White;
    board.update_board_state(&GameMove::new(black, (4,4), 0));
    board.update_board_state(&GameMove::new(white, (3,4), 0));
    board.update_board_state(&GameMove::new(white, (5,4), 0));
    board.update_board_state(&GameMove::new(white, (4,3), 0));
    board.update_board_state(&GameMove::new(white, (4,5), 0));
    assert_eq!(Stone::Empty, board.stone_at(4,4));
    assert_eq!(white, board.stone_at(3,4));
    assert_eq!(white, board.stone_at(5,4));
    assert_eq!(white, board.stone_at(4,3));
    assert_eq!(white, board.stone_at(4,5));
}
