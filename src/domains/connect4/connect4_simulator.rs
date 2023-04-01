use crate::core::initial_state_generator::InitialStateGenerator;
use crate::core::reward::{Reward, get_adversarial_draw, get_adversarial_p1_win, get_adversarial_p1_loss, AdversarialReward};
use crate::core::simulator::{Simulator, LegalActions};

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
    column_heights_cache: HashMap<Connect4State, [u8; BOARD_WIDTH]>,
    rewards_cache: HashMap<Connect4State, Vec<Reward>>,
    legal_actions_cache: HashMap<Connect4State, Vec<LegalActions<Connect4Action>>>,
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
            column_heights_cache: HashMap::new(),
            rewards_cache: HashMap::new(),
            legal_actions_cache: HashMap::new(),
        }
    }

    fn calculate_column_heights(&mut self, state: &Connect4State) -> [u8; BOARD_WIDTH] {
        let column_heights = match self.column_heights_cache.get(state) {
            Some(column_heights) => column_heights.clone(),
            None => calculate_column_heights(state),
        };
        self.column_heights_cache.clear();
        self.column_heights_cache.insert(state.clone(), column_heights);
        return column_heights;
    }
}

impl Simulator<Connect4State, Connect4Action> for Connect4Simulator {

    fn calculate_rewards(&mut self, state: &Connect4State) -> Vec<Reward> {
        let cache = self.rewards_cache.get(state);
        let rewards = match cache {
            Some(rewards) => rewards.clone(),
            None => calculate_rewards(state)
        };
        self.rewards_cache.clear();
        self.rewards_cache.insert(state.clone(), rewards.clone());
        return rewards
    }

    fn calculate_legal_actions(&mut self, state: &Connect4State) -> Vec<LegalActions<Connect4Action>> {
        let legal_actions = match self.legal_actions_cache.get(state) {
            Some(legal_actions) => legal_actions.clone(),
            None => calculate_legal_actions(state, &calculate_rewards(state), self.calculate_column_heights(state)),
        };
        self.legal_actions_cache.clear();
        self.legal_actions_cache.insert(state.clone(), legal_actions.clone());
        return legal_actions;
    }

    fn state_transition(&mut self, state: &Connect4State, actions: &HashMap<usize, Connect4Action>) -> Connect4State {
        let mut state = state.clone();
        let agent_turn: usize = if state.player_1_turn() { 0 } else { 1 };
        let action = actions.get(&agent_turn).unwrap();
        let legal_actions = self.calculate_legal_actions(&state);
        let mut column_heights = self.calculate_column_heights(&state);
        if legal_actions[agent_turn].iter().find(|&a| a == action) == None {
            panic!("Illegal action, {action}, from state, {state}");
        }
        column_heights[action.location as usize] += 1;
        if state.player_1_turn() {
            state.bit_board_1 = state.bit_board_1 ^ (1 << column_heights[action.location as usize]);
        } else {
            state.bit_board_2 = state.bit_board_2 ^ (1 << column_heights[action.location as usize]);
        }
        return state;
    }
}

impl InitialStateGenerator for Connect4Simulator {
    type S = Connect4State;
    fn generate_initial_state(&self) -> Self::S {
        Connect4State { bit_board_1: 0, bit_board_2: 0 }
    }
}

fn calculate_legal_actions(
    state: &Connect4State,
    rewards: &Vec<Reward>,
    column_heights: [u8; BOARD_WIDTH], 
) -> Vec<LegalActions<Connect4Action>> {
    let mut legal_actions = vec![LegalActions::<Connect4Action>::new(); N_PLAYERS];
    if rewards[0] == Reward::Adversarial(AdversarialReward::Draw) {
        let agent_turn = if state.player_1_turn() { 0 } else { 1 };
        (0..BOARD_WIDTH)
            .filter(|&x| { 1 << column_heights[x] & ABOVE_TOP_ROW == 0 })
            .for_each(|x| { legal_actions[agent_turn].insert(Connect4Action { location: x as u8 }); });
    }
    return legal_actions;
}

fn calculate_rewards(state: &Connect4State) -> Vec<Reward> {
    let height = BOARD_HEIGHT;
    for i in 0..N_PLAYERS {
        let bit_board = if i == 0 { state.bit_board_1 } else { state.bit_board_2 };
        let diagonal1 = bit_board & (bit_board >> height);
        let horizontal = bit_board & (bit_board >> height + 1);
        let diagonal2 = bit_board & (bit_board >> height + 2);
        let vertical = bit_board & (bit_board >> 1);
        if diagonal1 & (diagonal1 >> 2 * height)| 
            (horizontal & (horizontal >> 2 * (height + 1))) | 
            (diagonal2 & (diagonal2 >> 2 * (height + 2))) |
            (vertical & (vertical >> 2)) != 0 {
            return if i == 0 { get_adversarial_p1_win() } else { get_adversarial_p1_loss() };
        }
    }
    return get_adversarial_draw();
}

fn calculate_column_heights(state: &Connect4State) -> [u8; BOARD_WIDTH] {
    let mut column_heights = [0u8; BOARD_WIDTH];
    let bit_board = state.bit_board_1 | state.bit_board_2;
    for i in 0..BOARD_WIDTH {
        column_heights[i] = ((BOARD_HEIGHT + 1) * i) as u8;
        while bit_board & (1 << column_heights[i]) != 0 {
            column_heights[i] += 1;
        }
    }
    return column_heights;
}
