use std::fmt;

use crate::core::simulator::Action;

/// Represents a slot location to place piece.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Connect4Action {
    pub location: u8,
}

impl Action for Connect4Action {}

impl fmt::Display for Connect4Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.location + 1)
    }
}
