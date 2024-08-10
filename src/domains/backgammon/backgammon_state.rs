use std::fmt;

use crate::core::simulator::State;

pub const N_DICE: usize = 2;
pub const N_DIE_FACES: usize = 6;
pub const N_LOCATIONS: usize = 26;

/// Represents a backgammon state as an array of byte locations.
/// @param locations each location is 0 if no pieces are at that location and positive for
///                 the number of pieces player 1 has there and negative for the number of
///                 pieces player 2 has there.
/// @param dice two values in range 0-5 that represent die faces 1 to 6 each; order does not matter
/// @param p1_turn player 1 turn
#[derive(Clone, Copy, fmt::Debug, Hash, PartialEq, Eq)]
pub struct BackgammonState {
    pub locations: [i8; N_LOCATIONS],
    pub dice: [u8; N_DICE],
    pub p1_turn: bool,
}

impl State for BackgammonState {}

impl fmt::Display for BackgammonState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!("{} - [{:?}][{:?}]\n", self.p1_turn as i8, self.dice[0] + 1, self.dice[1] + 1);
        for i in (12..7).rev() {
            if self.locations[i] >= 0 {
                output.push_str(" ");
            }
            output.push_str(&format!("{:?}", self.locations[i]));
        }
        output.push_str("|");
        for i in (6..1).rev() {
            if self.locations[i] >= 0 {
                output.push_str(" ");
            }
            output.push_str(&format!("{:?}", self.locations[i]));
        }
        output.push_str(&format!(" [{:?}]\n", self.locations[0]));
        output.push_str("------------|------------\n");
        for i in 13..18 {
            if self.locations[i] >= 0 {
                output.push_str(" ");
            }
            output.push_str(&format!("{:?}", self.locations[i]));
        }
        output.push_str("|");
        for i in 19..25 {
            if self.locations[i] >= 0 {
                output.push_str(" ");
            }
            output.push_str(&format!("{:?}", self.locations[i]));
        }
        output.push_str(&format!(" [{:?}]", self.locations[25]));
        write!(f, "{}", output)
    }
}