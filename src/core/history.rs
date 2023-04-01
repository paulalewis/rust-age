use super::simulator::Action;
use super::simulator::State;

use std::collections::HashMap;

/// Keeps track of state transition history.
pub struct History<S : State, A : Action> {
    nodes: Vec<HistoryNode<S, A>>,
}

/// A history node can be a state or set of actions.
/// The history alternates between states and actions.
pub enum HistoryNode<S : State, A : Action> {
    State(S),
    Actions(HashMap<usize, A>),
}

impl <S : State, A : Action> History<S, A> {
    /// Create a new history with an initial state.
    pub fn new(initial_state: S) -> Self {
        History {
            nodes: vec![HistoryNode::State(initial_state)],
        }
    }

    /// Add the next state and the actions taken by each agent
    /// to arrive at that state.
    pub fn push(&mut self, state: S, actions: HashMap<usize, A>) {
        self.nodes.push(HistoryNode::Actions(actions));
        self.nodes.push(HistoryNode::State(state));
    }

    /// Remove the last state and actions from the history.
    /// If the history is already at an initial state, then
    /// this function will panic.
    pub fn pop(&mut self) {
        if self.nodes.len() < 2 {
            panic!("Cannot remove last state from history");
        }
        self.nodes.pop();
        self.nodes.pop();
    }

    pub fn peek(&self) -> (&S, Option<&HashMap<usize, A>>) {
        let state = match self.nodes.last() {
            Some(HistoryNode::State(state)) => Some(state),
            _ => None,
        };
        let actions = match self.nodes.get(self.nodes.len() - 2) {
            Some(HistoryNode::Actions(actions)) => Some(actions),
            _ => None,
        };
        return (state.unwrap(), actions);
    }
}
