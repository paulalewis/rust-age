use std::fmt;
use std::hash::Hash;
use std::vec::Vec;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

pub trait Action : Clone + fmt::Display + Hash + Eq {}
pub trait State : Clone + fmt::Display + Hash + Eq {
    fn get_current_player_ids(&self) -> Vec<usize>;
}

pub struct LegalActions<A : Action>(pub HashSet<A>);

impl <A : Action> fmt::Display for LegalActions<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[")?;
        self.0.iter().fold(Ok(()), |result, action| {
            result.and_then(|_| writeln!(f, "{}", action))
        });
        write!(f, "]")
    }
}

/// A simulator controls the state transitions of a given domain
/// and is associated with a domain specific state and action type.
pub trait Simulator<S : State, A : Action> : Clone {
    /// Generates an initial state for the domain.
    /// 
    /// The initial state returned is not necessarily always
    /// the same state.
    /// 
    /// ### Return Value
    /// 
    /// Returns an inital state in the domain.
    fn initial_state(&self) -> S;

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
    fn calculate_rewards(&self, state: &S) -> Vec<i32>;

    /// @param state the state from which to calculate rewards
    /// @return list of legal actions for each player
    fn calculate_legal_actions(&self, state: &S) -> Vec<LegalActions<A>>;

    /// Transition from the current state to the next state
    /// given a set of player actions.
    /// 
    /// @param actions map of actions to be performed by each player
    fn state_transition(&mut self, state: &S, actions: &HashMap<usize, A>) -> S;
    
    /// The number of players in this domain.
    /// 
    /// The number of players can be affected by
    /// the current state, e.g. a multiplayer
    /// game where one player is eliminated but
    /// play continues.
    fn number_of_players(&self, state: &S) -> usize {
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
    fn is_terminal_state(&self, state: &S) -> bool {
        let legal_actions = self.calculate_legal_actions(state);
        let result = legal_actions.iter().find(|a| a.0.is_empty());
        return match result {
            Some(_) => false,
            None => true,
        }
    }
}
