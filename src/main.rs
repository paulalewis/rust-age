use abstract_game_engine::core::agent::Agent;
use abstract_game_engine::core::simulator::Simulator;
use abstract_game_engine::core::agent::RandomAgent;
use abstract_game_engine::core::agent::IoAgent;
use abstract_game_engine::core::simulator::State;
use abstract_game_engine::domains::connect4::connect4_action::Connect4Action;
use abstract_game_engine::domains::connect4::connect4_simulator::Connect4Simulator;
use abstract_game_engine::domains::yahtzee::yahtzee_action::YahtzeeAction;
use abstract_game_engine::domains::yahtzee::yahtzee_simulator::YahtzeeSimulator;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use std::collections::HashMap;
use std::io;
use std::fmt;

fn main() {
    let domains = vec![Domain::Connect4, Domain::Yahtzee];
    let domain = select_domain(domains);
    let mut random_agent = RandomAgent::new();
    let mut io_agent = IoAgent::new();

    println!("{domain}");

    match domain {
        Domain::Connect4 => {
            let mut simulator = Connect4Simulator::new();
            let mut current_state = simulator.generate_initial_state();
            while !simulator.is_terminal_state(&current_state) {
                println!("{}", current_state);
                let mut selected_actions: HashMap<usize, Connect4Action> = HashMap::new();
                for player_id in current_state.get_current_player_ids() {
                    let action = io_agent.select_action(player_id, &current_state, &mut simulator);
                    selected_actions.insert(0, action);
                }
                current_state = simulator.state_transition(&current_state, &selected_actions);
            }
            print!("Game Over - {:?}", simulator.calculate_rewards(&current_state));
        }
        Domain::Yahtzee => {
            let seed = select_seed();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let mut simulator = YahtzeeSimulator::new(&mut rng);
            let mut current_state = simulator.generate_initial_state();
            while !simulator.is_terminal_state(&current_state) {
                println!("{}", current_state);
                let mut selected_actions: HashMap<usize, YahtzeeAction> = HashMap::new();
                for player_id in current_state.get_current_player_ids() {
                    let action = random_agent.select_action(player_id, &current_state, &mut simulator);
                    selected_actions.insert(player_id, action);
                }
                current_state = simulator.state_transition(&current_state, &selected_actions);
            }
            print!("Game Over - {:?}", simulator.calculate_rewards(&current_state));
        }
    };
}

#[derive(Clone, Copy)]
enum Domain {
    Connect4,
    Yahtzee,
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Domain::Connect4 => write!(f, "Connect 4"),
            Domain::Yahtzee => write!(f, "Yahtzee"),
        }
    }
}

fn select_domain(domains: Vec<Domain>) -> Domain {
    let mut input = String::new();

    println!("Select domain:");

    for (i, domain) in domains.iter().enumerate() {
        let j = i + 1;
        println!("{j} {domain}");
    }
    
    loop {
        io::stdin().read_line(&mut input).unwrap();
        let value = input.trim().parse::<usize>();
        let choice = match value {
            Ok(value) => domains.get(value - 1),
            Err(_) => None,
        };
        match choice {
            Some(domain) => break domain.clone(),
            None => {
                println!("Invalid input: {}", input);
                input.clear();
            },
        }
    }
}

fn select_seed() -> u64 {
    let mut input = String::new();
    println!("Select a seed (u64 value, or press enter to select random seed):");
    loop {
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u64>() {
            Ok(seed) => break seed,
            Err(_) => {
                let seed = rand::random::<u64>();
                println!("Selecting a random seed = {seed}");
                break seed;
            },
        }
    }
}
