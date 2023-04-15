use go::game::Game;
use go::*;
use std::collections::VecDeque;

struct MockUi {
    inputs: VecDeque<(usize, usize)>,
}

impl MockUi {
    fn new(inputs: Vec<(usize, usize)>) -> Self {
        Self {
            inputs: inputs.into(),
        }
    }
}

impl UserInterface for MockUi {
    fn view(&mut self, _board: &Board) -> anyhow::Result<()> {
        Ok(())
    }

    fn input(&mut self) -> anyhow::Result<UserAction> {
        if let Some((row, col)) = self.inputs.pop_front() {
            return Ok(UserAction::Move(row, col));
        }

        Ok(UserAction::Quit)
    }
}

#[test]
fn it_works() {
    let ui = MockUi::new(vec![(0, 0), (1, 1), (2, 2)]);

    let mut game = Game::new_game(3, 3, ui);
    game.start_game().unwrap();

    let state = game.board.get_state();
    assert_eq!(state[0][0], Stone::Black);
    assert_eq!(state[1][1], Stone::White);
    assert_eq!(state[2][2], Stone::Black);
}
