use crate::game_logic::stone::Stone;

/// Represents a move made in the game.
///
/// The `GameMove` struct encapsulates the details of a move made by a player during the game.
/// It stores information about the color of the stone played, the position on the board where the stone was placed,
/// and the move number corresponding to when the move was made.
///
/// # Fields
///
/// * `stone`: The color of the stone played in the move.
/// * `pos`: The position on the board where the stone was placed, represented as a tuple of `(row, col)`.
/// * `move_number`: The move number that this move corresponds to in the game's sequence of moves.
///
/// The `GameMove` struct is used to track and represent individual moves within the game, allowing for recording,
/// replaying, and analyzing the sequence of moves made by players. It serves as a fundamental data structure
/// in capturing the state transitions of the game.
///
/// Note that this struct does not enforce any game rules or validity of the move. It solely provides a convenient
/// container to store and retrieve information about each move made during the game.
pub struct GameMove {
    pub stone: Stone,
    pub pos: (usize, usize),
    pub move_number: usize,
}

impl GameMove {
    // Should be y,x
    pub fn new(stone: Stone, pos: (usize, usize), move_number: usize) -> Self {
        GameMove {
            stone,
            pos,
            move_number,
        }
    }
}
