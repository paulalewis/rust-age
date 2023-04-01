use super::simulator::Action;
use super::simulator::State;

use std::collections::HashMap;

/// Keeps track of state transition history.
pub struct History<S : State, A : Action> {
    nodes: Vec<HistoryNode<S, A>>,
}

pub struct HistoryNode<S : State, A : Action> {
    pub state: S,
    pub actions: HashMap<usize, A>,
}

impl <S : State, A : Action> History<S, A> {
    /// Create a new history with an initial state.
    pub fn new(initial_state: S) -> Self {
        History {
            nodes: vec![HistoryNode { state: initial_state, actions: HashMap::new() }],
        }
    }

    /// Clear the history.
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// Add the next state and the actions taken by each agent
    /// to arrive at that state.
    /// @param state the current state
    /// @param actions the actions taken by each player to end up in the current state
    pub fn add(&mut self, state: S, actions: HashMap<usize, A>) {
        self.nodes.push(HistoryNode { state, actions });
    }
}
