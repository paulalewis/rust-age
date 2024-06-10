use std::fmt;
use std::hash::Hash;
use std::vec::Vec;
use std::collections::hash_map::HashMap;
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
pub trait Simulator<S : State, A : Action> {
    /// Generates an initial state for the domain.
    /// 
    /// The initial state returned is not necessarily always
    /// the same state.
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
    /// Returns a reward value for each player that can be
    /// indexed by the player ID.
    fn calculate_rewards(&mut self, state: &S) -> Vec<Reward>;

    /// @param state the state from which to calculate rewards
    /// @return list of legal actions for each player
    fn calculate_legal_actions(&mut self, state: &S) -> Vec<LegalActions<A>>;

    /// Transition from the current state to the next state
    /// given a set of player actions.
    /// 
    /// @param actions map of actions to be performed by each player
    fn state_transition(&mut self, state: &S, actions: &HashMap<usize, A>) -> S;
    
    /// The player IDs of the current players in the domain.
    fn get_current_player_ids(&self, state: &S) -> Vec<usize>;
    
    /// The number of players in this domain.
    /// 
    /// The number of players can be affected by
    /// the current state, e.g. a multiplayer
    /// game where one player is eliminated but
    /// play continues.
    fn number_of_players(&mut self, state: &S) -> usize {
        return self.calculate_legal_actions(state).len();
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

        fn state_transition(&mut self, _: &TestState, _: &HashMap<usize, TestAction>) -> TestState {
            TestState
        }
        
        fn get_current_player_ids(&self, _state: &TestState) -> Vec<usize> {
            vec![0, 1]
        }
    }
}
