use std::fmt::{Display, self};

use crate::core::simulator::State;

use super::constants::{BOARD_HEIGHT, BOARD_WIDTH};
use super::util::count_ones;

/// Connect 4 board state.
/// 
/// State is represented by a bit board described below:
/// .  .  .  .  .  .  . Row above top row
/// 5 12 19 26 33 40 47
/// 4 11 18 25 32 39 46
/// 3 10 17 24 31 38 45
/// 2  9 16 23 30 37 44
/// 1  8 15 22 29 36 43
/// 0  7 14 21 28 35 42
#[derive(Clone, fmt::Debug, Hash, PartialEq, Eq)]
pub struct Connect4State {
    pub bit_board: [u64; 2],
}

impl Connect4State {
    pub fn player_1_turn(&self) -> bool {
        return count_ones(self.bit_board[0]) <= count_ones(self.bit_board[1]);
    }
}

impl State for Connect4State {}

impl Display for Connect4State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..BOARD_HEIGHT).rev() {
            let mut j = i;
            while j < (BOARD_HEIGHT + 1) * BOARD_WIDTH {
                let mask = 1 << j;
                if (self.bit_board[0] & mask) != 0 {
                    write!(f, "X")?;
                } else if (self.bit_board[1] & mask) != 0 {
                    write!(f, "O")?;
                } else {
                    write!(f, "-")?;
                }
                j += BOARD_HEIGHT + 1;
            }
            if i != 0 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect4_state_to_string_empty_board() {
        let state = Connect4State { bit_board: [0, 0] };
        let expected = "-------\n\
                              -------\n\
                              -------\n\
                              -------\n\
                              -------\n\
                              -------";
        assert_eq!(state.to_string(), expected);
    }
    
    #[test]
    fn connect4_state_to_string() {
        let state = Connect4State { bit_board: [0b10_0000001_0000000_0000000_0000000, 0b1_0000000_0000000_0000001_0000000] };
        let expected = "-------\n\
                              -------\n\
                              -------\n\
                              -------\n\
                              ----X--\n\
                              -O-XO--";
        assert_eq!(state.to_string(), expected);
    }

    #[test]
    fn player_1_turn_empty() {
        let state = Connect4State { bit_board: [0, 0] };
        assert_eq!(state.player_1_turn(), true);
    }
}