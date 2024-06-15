use std::fmt;
use std::hash::Hash;
use std::vec::Vec;
use std::collections::hash_set::HashSet;

use super::reward::Reward;

pub trait Action : Clone + fmt::Debug + fmt::Display + Hash + Eq {}
pub trait State : Clone + fmt::Debug + fmt::Display + Hash + Eq {}

#[derive(Clone)]
pub struct LegalActions<A : Action>(pub HashSet<A>);

impl <A : Action> LegalActions<A> {
    pub fn new() -> Self {
        LegalActions(HashSet::<A>::new())
    }

    pub fn insert(&mut self, action: A) {
        self.0.insert(action);
    }

    pub fn iter(&self) -> std::collections::hash_set::Iter<A> {
        self.0.iter()
    }
}

impl <A : Action> fmt::Display for LegalActions<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[")?;
        self.0.iter().fold(Ok(()), |result, action| {
            result.and_then(|_| writeln!(f, "{}", action))
        })?;
        write!(f, "]")
    }
}

/// A simulator controls the state transitions of a given domain
/// and is associated with a domain specific state and action type.
/// The number of players should be fixed even if a player has
/// been eleminated from the game.
pub trait Simulator<S : State, A : Action> {
    /// Generates an initial state for the domain.
    /// 
    /// ### Return Value
    /// 
    /// Returns an inital state in the domain.
    fn generate_initial_state(&mut self) -> S;

    /// This is the rewards function for the given domain.
    /// 
    /// ### Arguments
    /// 
    /// * `state` - The state from which to calculate rewards.
    /// 
    /// ### Return Value
    /// 
    /// Returns a reward value for each player.
    fn calculate_rewards(&mut self, state: &S) -> Vec<Reward>;

    /// Calculates the legal actions for each player in the domain.
    /// 
    /// ### Arguments
    /// 
    /// * `state` - The state from which to calculate legal actions.
    /// 
    /// ### Return Value
    /// 
    /// Returns a list of legal actions for each player.
    fn calculate_legal_actions(&mut self, state: &S) -> Vec<LegalActions<A>>;

    /// Transition from the current state to the next state
    /// given a set of player actions maped from each player ID.
    /// 
    /// ### Arguments
    /// 
    /// * `state` - The state from which to transition.
    /// * `actions` - Map of actions to be performed by each player.
    /// 
    /// ### Return Value
    /// 
    /// Returns the next state after the actions have been performed.
    fn state_transition(&mut self, state: &S, actions: &Vec<Option<A>>) -> S;
    
    /// The total number of players in this domain.
    /// This should be fixed throughout the entire game.
    fn number_of_players(&mut self) -> usize;

    /// Checks if the given state transition is valid.
    /// The state transition is valid if each player that
    /// has legal actions has selected a legal action and
    /// each player that has no legal actions has not selected
    /// an action.
    /// 
    /// ### Arguments
    /// 
    /// * `state` - The state from which to transition.
    /// * `actions` - Map of actions to be performed by each player.
    /// 
    /// ### Return Value
    /// 
    /// True if the state transition is valid.
    fn check_valid_state_transition(&mut self, state: &S, actions: &Vec<Option<A>>) -> Result<(), String> {
        let all_legal_actions = self.calculate_legal_actions(state);
        if actions.len() != self.number_of_players() {
            return Err(format!("actions length is {}, while number of players is {}", actions.len(), self.number_of_players()));
        }
        for player_id in 0..self.number_of_players() {
            let legal_actions = &all_legal_actions[player_id];
            let action = &actions[player_id];
            if (legal_actions.0.is_empty() && !action.is_none()) ||
                    (!legal_actions.0.is_empty() && action.is_none()) {
                return Err(format!("player {} has illegal action", player_id));
            }
        }
        return Ok(());
    }

    /// A state is terminal if no player has any
    /// legal actions from the current state.
    /// 
    /// ### Arguments
    /// 
    /// * `state` - The state to check if terminal.
    /// 
    /// ### Returns
    /// 
    /// True if no player has any legal actions from the given state.
    fn is_terminal_state(&mut self, state: &S) -> bool {
        let legal_actions = self.calculate_legal_actions(state);
        let result = legal_actions.iter().find(|a| !a.0.is_empty());
        return match result {
            Some(_) => false,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::reward::ADVERSARIAL_DRAW;

    use super::*;

    #[test]
    fn is_terminal_state_no_legal_actions() {
        let mut simulator = TestSimulator {
            legal_actions: vec![LegalActions::new(), LegalActions::new()],
        };
        assert!(simulator.is_terminal_state(&TestState));
    }
    
    #[test]
    fn is_terminal_state_one_legal_action() {
        let legal_actions_p2 = {
            let mut legal_actions = LegalActions::new();
            legal_actions.insert(TestAction);
            legal_actions
        };
        let mut simulator = TestSimulator {
            legal_actions: vec![LegalActions::new(), legal_actions_p2],
        };
        assert!(!simulator.is_terminal_state(&TestState));
    }

    #[test]
    fn check_valid_state_transition_actions_vec_too_small() {
        let mut simulator = TestSimulator {
            legal_actions: vec![LegalActions::new(), LegalActions::new()],
        };
        let actions = vec![];
        assert!(simulator.check_valid_state_transition(&TestState, &actions).is_err());
    }

    #[test]
    fn check_valid_state_transition_actions_vec_too_large() {
        let mut simulator = TestSimulator {
            legal_actions: vec![LegalActions::new(), LegalActions::new()],
        };
        let actions = vec![None, None, None];
        assert!(simulator.check_valid_state_transition(&TestState, &actions).is_err());
    }
    
    #[test]
    fn check_valid_state_transition_actions_vec_all_none() {
        let mut simulator = TestSimulator {
            legal_actions: vec![LegalActions::new(), LegalActions::new()],
        };
        let actions = vec![None, None];
        assert!(simulator.check_valid_state_transition(&TestState, &actions).is_ok());
    }

    #[derive(Clone, fmt::Debug, Hash, PartialEq, Eq)]
    struct TestState;

    impl State for TestState {}

    impl fmt::Display for TestState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestState")
        }
    }

    #[derive(Clone, fmt::Debug, Hash, PartialEq, Eq)]
    struct TestAction;

    impl Action for TestAction {}
    
    impl fmt::Display for TestAction {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "TestAction")
        }
    }

    struct TestSimulator {
        legal_actions: Vec<LegalActions<TestAction>>,
    }

    impl Simulator<TestState, TestAction> for TestSimulator {

        fn generate_initial_state(&mut self) -> TestState {
            TestState
        }

        fn calculate_rewards(&mut self, _: &TestState) -> Vec<Reward> {
            ADVERSARIAL_DRAW.to_vec()
        }

        fn calculate_legal_actions(&mut self, _: &TestState) -> Vec<LegalActions<TestAction>> {
            self.legal_actions.clone()
        }

        fn state_transition(&mut self, _: &TestState, _: &Vec<Option<TestAction>>) -> TestState {
            TestState
        }
        
        fn number_of_players(&mut self) -> usize {
            2
        }
    }
}
