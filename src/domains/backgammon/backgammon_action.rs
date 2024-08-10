use std::fmt;

use crate::core::simulator::Action;

#[derive(Clone, Copy, fmt::Debug, Hash, PartialEq, Eq)]
pub struct BackgammonAction {
    moves: [BackgammonMove; 2],
}

impl fmt::Display for BackgammonAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str("[ ");
        for move_ in &self.moves {
            output.push_str(&format!("{:?} ", move_));
        }
        output.push_str("]");
        write!(f, "{}", output)
    }
}

impl Action for BackgammonAction {}

/// Represents moving a single piece from one location to another. It is a
/// partial action as an action may be made up of multiple moves.
#[derive(Clone, Copy, fmt::Debug, Hash, PartialEq, Eq)]
pub struct BackgammonMove {
    from: u8,
    distance: u8,
}

impl fmt::Display for BackgammonMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.from, self.distance)
    }
}