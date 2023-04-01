use crate::simulator::{Simulator, LegalActions};

use super::connect4_constants::{BOARD_WIDTH, BOARD_HEIGHT};
use super::connect4_state::Connect4State;
use super::connect4_action::Connect4Action;

use std::collections::HashMap;
        
const N_PLAYERS: usize = 2;
const ALL_LOCATIONS: u64 = (1 << (BOARD_HEIGHT + 1) * BOARD_WIDTH) - 1;
const FIRST_COLUMN: u64 = (1 << BOARD_HEIGHT + 1) - 1;
const BOTTOM_ROW: u64 = ALL_LOCATIONS / FIRST_COLUMN;
const ABOVE_TOP_ROW: u64 = BOTTOM_ROW << BOARD_HEIGHT;

#[derive(Clone)]
pub struct Connect4Simulator {
    action_pool: [Connect4Action; BOARD_WIDTH],
}

impl Connect4Simulator {
    pub fn new() -> Self {
        Connect4Simulator {
            action_pool: [
                Connect4Action { location: 0 },
                Connect4Action { location: 1 },
                Connect4Action { location: 2 },
                Connect4Action { location: 3 },
                Connect4Action { location: 4 },
                Connect4Action { location: 5 },
                Connect4Action { location: 6 },
            ],
        }
    }
}

impl Simulator<Connect4State, Connect4Action> for Connect4Simulator {

    fn initial_state(&self) -> Connect4State {
        Connect4State { bit_board_1: 0, bit_board_2: 0 }
    }

    fn calculate_rewards(&self, state: &Connect4State) -> Vec<i32> {
        todo!()
    }

    fn calculate_legal_actions(&self, state: &Connect4State) -> Vec<LegalActions<Connect4Action>> {
        todo!()
    }

    fn state_transition(&mut self, state: &Connect4State, actions: &HashMap<usize, Connect4Action>) -> Connect4State {
        todo!()
    }

    fn number_of_players(&self, state: &Connect4State) -> usize {
        return self.calculate_legal_actions(state).len();
    }

    fn is_terminal_state(&self, state: &Connect4State) -> bool {
        let legal_actions = self.calculate_legal_actions(state);
        let result = legal_actions.iter().find(|a| a.0.is_empty());
        return match result {
            Some(_) => false,
            None => true,
        }
    }
}
