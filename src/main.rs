use abstract_game_engine::core::agent::Agent;
use abstract_game_engine::core::simulator::Simulator;
use abstract_game_engine::core::agent::RandomAgent;
use abstract_game_engine::core::agent::IoAgent;
use abstract_game_engine::domains::connect4::connect4_action::Connect4Action;
use abstract_game_engine::domains::connect4::connect4_simulator::Connect4Simulator;
use abstract_game_engine::domains::yahtzee::yahtzee_action::YahtzeeAction;
use abstract_game_engine::domains::yahtzee::yahtzee_simulator::YahtzeeSimulator;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use std::collections::HashMap;
use std::io;

fn main() {
    let domain = select_domain();
    let mut random_agent = RandomAgent::new();
    let mut io_agent = IoAgent::new();

    match domain {
        Domain::Connect4 => {
            println!("Connect 4");
            let mut simulator = Connect4Simulator::new();
            let mut current_state = simulator.generate_initial_state();
            loop {
                println!("{}", current_state);
                let action = io_agent.select_action(0, &current_state, &mut simulator);
                let mut selected_actions: HashMap<usize, Connect4Action> = HashMap::new();
                selected_actions.insert(0, action);
                current_state = simulator.state_transition(&current_state, &selected_actions);
            }
        }
        Domain::Yahtzee => {
            println!("Yahtzee");
            let seed = select_seed();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let mut simulator = YahtzeeSimulator::new(&mut rng);
            let mut current_state = simulator.generate_initial_state();
            loop {
                println!("{}", current_state);
                let action = io_agent.select_action(0, &current_state, &mut simulator);
                let mut selected_actions: HashMap<usize, YahtzeeAction> = HashMap::new();
                selected_actions.insert(0, action);
                current_state = simulator.state_transition(&current_state, &selected_actions);
            }
        }
    };
}
enum Domain {
    Connect4,
    Yahtzee,
}

fn select_domain() -> Domain {
    let mut input = String::new();
    println!("Select a domain:");
    println!("1) Connect 4");
    println!("2) Yatzee");
    
    loop {
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => break Domain::Connect4,
            "2" => break Domain::Yahtzee,
            _ => {
                println!("Invalid input: {}", input);
                input.clear();
            },
        }
    }
}

fn select_seed() -> u64 {
    let mut input = String::new();
    println!("Select a seed (u64 value):");
    loop {
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<u64>() {
            Ok(seed) => break seed,
            Err(_) => {
                println!("Invalid input: {}", input);
                let seed = rand::random::<u64>();
                println!("Selecting a random seed = {seed}");
                break seed;
            },
        }
    }
}
