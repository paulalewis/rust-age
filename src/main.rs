mod agent;
mod domain;
mod simulator;
mod domains;
mod player;

use agent::Agent;
use domain::Domain;
use simulator::Action;
use simulator::State;

use crate::agent::RandomAgent;
use crate::agent::IoAgent;
use crate::domains::connect4::connect4_action::Connect4Action;
use crate::domains::connect4::connect4_simulator::Connect4Simulator;
use crate::domains::yahtzee::yahtzee_action::YahtzeeAction;
use crate::domains::yahtzee::yahtzee_score_category::YahtzeeScoreCategory;
use crate::domains::yahtzee::yahtzee_action::YahtzeeAction::{YahtzeeRollAction, YahtzeeSelectAction};
use crate::domains::yahtzee::yahtzee_simulator::YahtzeeSimulator;
use crate::player::HumanPlayer;
use crate::player::Player;
use crate::player::RandomPlayer;
use crate::simulator::Simulator;

use std::collections::HashMap;
use std::io;

fn main() {
    let domain = select_domain();
    let random_agent = RandomAgent::new();
    let mut io_agent = IoAgent::new();

    match domain {
        Domain::Connect4 => {
            println!("Connect 4");
            let simulator = Connect4Simulator::new();
            let mut current_state = simulator.initial_state();
            let human_player = HumanPlayer::new(0, io_agent);
            let random_player = RandomPlayer::new(1, random_agent);
        }
        Domain::Yahtzee => {
            println!("Yahtzee");
            let mut simulator = YahtzeeSimulator::new();
            let mut current_state = simulator.initial_state();
            // let human_player = HumanPlayer::new(0, io_agent);
            loop {
                println!("{}", current_state);
                let action = io_agent.select_action(0, &current_state, &simulator);
                match action {
                    Some(yahtzeeAction) => {
                        let mut selected_actions: HashMap<usize, YahtzeeAction> = HashMap::new();
                        selected_actions.insert(0, yahtzeeAction);
                        current_state = simulator.state_transition(&current_state, &selected_actions);
                    }
                    None => {
                        println!("Invalid action");
                    }
                }
            }
        }
    };
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
