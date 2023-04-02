use std::fmt;

use crate::core::simulator::Action;

/// Represents a slot location to place a piece.
#[derive(Clone, fmt::Debug, Hash, PartialEq, Eq)]
pub struct Connect4Action {
    pub location: u8,
}

impl Action for Connect4Action {}

impl fmt::Display for Connect4Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.location + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect4_action_to_string() {
        let action = Connect4Action { location: 0 };
        assert_eq!(action.to_string(), "(1)");
    }
}