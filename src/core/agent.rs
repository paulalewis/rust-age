use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::io;

use super::simulator::Action;
use super::simulator::Simulator;
use super::simulator::State;

/// An agent interacts in a domain by selecting
/// an action from a list of legal actions for
/// a player from the current state.
pub trait Agent {
    /// Selects an action for a given player.
    /// 
    /// ### Arguments
    /// 
    /// * `player_id` - The ID that indicates which player tha agent is using to select an action.
    /// * `state` - The current domain state.
    /// * `simulator` - The simulator that determines action outcomes in the domain.
    /// 
    /// ### Return Value
    /// 
    /// The selected action from the current state
    /// or None, if the agent had no legal actions to choose.
    // fn select_action(&self, player_id: usize, state: &Box<dyn State>, simulator: &dyn Simulator) -> Option<Box<dyn Action>>;
    fn select_action<S : State, A : Action, I: Simulator<S, A>>(&mut self, player_id: usize, state: &S, simulator: &mut I) -> Option<A>;
}

pub struct IoAgent {}

impl IoAgent {
    pub fn new() -> Self {
        IoAgent {}
    }
}

impl Agent for IoAgent {
    fn select_action<S : State, A : Action, I: Simulator<S, A>>(&mut self, player_id: usize, state: &S, simulator: &mut I) -> Option<A> {
        let mut input = String::new();

        loop {
            let player_legal_actions = &simulator.calculate_legal_actions(&state)[player_id];

            println!("Select an action:\n{}", player_legal_actions);

            io::stdin().read_line(&mut input).unwrap();

            match player_legal_actions.0.iter().find(|action| { action.to_string() == input.trim() }) {
                Some(action) => break Some(action.clone()),
                None => {
                    println!("Not a legal action: {}", input);
                    input.clear();
                },
            }
        }
    }
}

pub struct RandomAgent {
    rng: ChaCha8Rng,
}

impl RandomAgent {
    pub fn new() -> Self {
        RandomAgent {
            rng: ChaCha8Rng::from_entropy(),
        }
    }
    
    pub fn with_seed(seed: u64) -> Self {
        RandomAgent {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
}

impl Agent for RandomAgent {
    fn select_action<S : State, A : Action, I: Simulator<S, A>>(&mut self, player_id: usize, state: &S, simulator: &mut I) -> Option<A> {
        let legal_actions = simulator.calculate_legal_actions(state);
        let player_actions_result = legal_actions.get(player_id);
        return match player_actions_result {
            Some(player_actions) => {
                return if player_actions.0.is_empty() {
                    None
                } else {
                    let index = self.rng.gen_range(0..player_actions.0.len());
                    let element = player_actions.0.iter().nth(index).unwrap();
                    Some(element.clone())
                }
            }
            None => None
        }
    }
}