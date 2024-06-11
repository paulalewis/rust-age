use rand::Rng;
use rand_chacha::ChaCha8Rng;
use std::fmt;
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
    fn select_action<S, A, I>(&mut self, player_id: usize, state: &S, simulator: &mut I) -> A where
        S: State,
        A: Action,
        I: Simulator<S, A>;
}

#[derive(Debug)]
pub enum DefaultAgents {
    /// Selects an action by prompting the user.
    Io,
    /// Selects a random action from the list of legal actions.
    Random(ChaCha8Rng),
}

impl Agent for DefaultAgents {
    fn select_action<S, A, I>(&mut self, player_id: usize, state: &S, simulator: &mut I) -> A where
        S: State,
        A: Action,
        I: Simulator<S, A>,
    {
        match self {
            DefaultAgents::Io => {
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
            },
            DefaultAgents::Random(rng) => {
                let player_legal_actions = &simulator.calculate_legal_actions(&state)[player_id];
                let random_index = rng.gen_range(0..player_legal_actions.0.len());
                player_legal_actions.0.iter().nth(random_index).expect("Index should always be in bounds.").clone()
            },
        }
    }
}

impl fmt::Display for DefaultAgents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DefaultAgents::Io => write!(f, "Io Agent"),
            DefaultAgents::Random(_) => write!(f, "Random Agent"),
        }
    }
}