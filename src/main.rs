use abstract_game_engine::core::agent::Agent;
use abstract_game_engine::core::agent::DefaultAgents;
use abstract_game_engine::core::simulator::Simulator;
use abstract_game_engine::domains::connect4::connect4_action::Connect4Action;
use abstract_game_engine::domains::connect4::connect4_simulator::Connect4Simulator;
use abstract_game_engine::domains::yahtzee::yahtzee_action::YahtzeeAction;
use abstract_game_engine::domains::yahtzee::yahtzee_simulator::YahtzeeSimulator;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use std::io;
use std::fmt;

fn main() {
    let domain = select_domain();

    println!("{domain}");

    let rewards = match domain {
        Domain::Connect4 => {
            let mut simulator = Connect4Simulator::new();
            let mut current_state = simulator.generate_initial_state();
            let mut agents: Vec<DefaultAgents> = select_agents(simulator.number_of_players());

            while !simulator.is_terminal_state(&current_state) {
                println!("{}", current_state);
                let mut selected_actions: Vec<Option<Connect4Action>> = Vec::new();
                let player_legal_actions = simulator.calculate_legal_actions(&current_state);
                for player_id in 0..simulator.number_of_players() {
                    if player_legal_actions[player_id].0.is_empty() {
                        selected_actions.push(None);
                    } else {
                        let action = agents[player_id].select_action(player_id, &current_state, &mut simulator);
                        selected_actions.push(Some(action));
                    }
                }
                current_state = simulator.state_transition(&current_state, &selected_actions);
            }
            simulator.calculate_rewards(&current_state)
        }
        Domain::Yahtzee => {
            let seed = select_seed();
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let mut simulator = YahtzeeSimulator::new(&mut rng);
            let mut current_state = simulator.generate_initial_state();
            let mut agents: Vec<DefaultAgents> = select_agents(simulator.number_of_players());
            
            while !simulator.is_terminal_state(&current_state) {
                println!("{}", current_state);
                let mut selected_actions: Vec<Option<YahtzeeAction>> = Vec::new();
                let player_legal_actions = simulator.calculate_legal_actions(&current_state);
                for player_id in 0..simulator.number_of_players() {
                    if player_legal_actions[player_id].0.is_empty() {
                        selected_actions.push(None);
                    } else {
                        let action = agents[player_id].select_action(player_id, &current_state, &mut simulator);
                        selected_actions.push(Some(action));
                    }
                }
                current_state = simulator.state_transition(&current_state, &selected_actions);
            }
            simulator.calculate_rewards(&current_state)
        }
    };
    
    print!("Game Over - {:?}", rewards);
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

fn select_domain() -> Domain {
    let domains = vec![Domain::Connect4, Domain::Yahtzee];
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

fn select_agents(number_of_players: usize) -> Vec<DefaultAgents> {
    let mut agents: Vec<DefaultAgents> = Vec::new();
    for player_id in 0..number_of_players {
        println!("Select Player {player_id} Agent");
        agents.insert(player_id, select_agent());
    }
    agents
}

fn select_agent() -> DefaultAgents {
    let mut input = String::new();

    println!("1 Random Agent");
    println!("2 Io Agent");
    
    loop {
        io::stdin().read_line(&mut input).unwrap();
        let value = input.trim().parse::<usize>();
        let choice = match value {
            Ok(value) => Some(value),
            Err(_) => None,
        };
        match choice {
            Some(1) => break DefaultAgents::Random(ChaCha8Rng::from_entropy()),
            Some(2) => break DefaultAgents::Io,
            Some(_) | None => {
                println!("Invalid input: {}", input);
                input.clear();
            },
        }
    }
}