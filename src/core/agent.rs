use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::io;

use super::reward::Reward;
use super::simulator::Action;
use super::simulator::Simulator;
use super::simulator::State;

/// An agent interacts in a domain by selecting
/// an action from a list of legal actions for
/// a player from the current state.
pub trait Agent {
    /// Selects an action for a given player.
    /// 
    /// Will panic if the player has no legal actions.
    /// 
    /// ### Arguments
    /// 
    /// * `player_id` - The ID that indicates which player tha agent is using to select an action.
    /// * `state` - The current domain state.
    /// * `simulator` - The simulator that determines action outcomes in the domain.
    /// 
    /// ### Return Value
    /// 
    /// The selected action from the current state.
    fn select_action<S : State, A : Action, R : Reward, I: Simulator<S, A, R>>(&mut self, player_id: usize, state: &S, simulator: &mut I) -> A;
}

pub struct IoAgent {}

impl IoAgent {
    pub fn new() -> Self {
        IoAgent {}
    }
}

impl Agent for IoAgent {
    fn select_action<S : State, A : Action, R : Reward, I: Simulator<S, A, R>>(&mut self, player_id: usize, state: &S, simulator: &mut I) -> A {
        let mut input = String::new();

        loop {
            let player_legal_actions = &simulator.calculate_legal_actions(&state)[player_id];

            println!("Select an action:\n{}", player_legal_actions);

            io::stdin().read_line(&mut input).unwrap();

            match player_legal_actions.0.iter().find(|action| { action.to_string() == input.trim() }) {
                Some(action) => break action.clone(),
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
    fn select_action<S : State, A : Action, R : Reward, I: Simulator<S, A, R>>(&mut self, player_id: usize, state: &S, simulator: &mut I) -> A {
        let player_legal_actions = &simulator.calculate_legal_actions(&state)[player_id];
        let random_index = self.rng.gen_range(0..player_legal_actions.0.len());
        player_legal_actions.0.iter().nth(random_index).expect("Index should always be in bounds.").clone()
    }
}
