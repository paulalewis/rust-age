use std::fmt::{Display, self};

use crate::core::simulator::State;

use super::connect4_constants::{BOARD_HEIGHT, BOARD_WIDTH};

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
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Connect4State {
    pub bit_board_1: u64,
    pub bit_board_2: u64,
}

impl Connect4State {
    fn player_1_turn(&self) -> bool {
        return Connect4State::count_ones(self.bit_board_1) <= Connect4State::count_ones(self.bit_board_2);
    }

    fn count_ones(value: u64) -> u8 {
        let mut count = 0;
        let mut new_value = value;
        for _ in 0..64 {
            if value & 1 == 1 {
                count += 1;
            }
            new_value = new_value >> 1;
        }
        return count;
    }
}

impl State for Connect4State {
    fn get_current_player_ids(&self) -> Vec<usize> {
        if self.player_1_turn() {
            return vec![0];
        } else {
            return vec![1];
        }
    }
}

impl Display for Connect4State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..BOARD_HEIGHT).rev() {
            let mut j = i;
            while j < (BOARD_HEIGHT + 1) * BOARD_WIDTH {
                let mask = 1 << j;
                if (self.bit_board_1 & mask) != 0 {
                    write!(f, "X")?;
                } else if (self.bit_board_2 & mask) != 0 {
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
