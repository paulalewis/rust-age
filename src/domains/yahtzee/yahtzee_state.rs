use std::fmt::Display;

use crate::simulator::State;

use super::{yahtzee_score_category::YahtzeeScoreCategory, yahtzee_constants::{N_VALUES, N_DICE}};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct YahtzeeState {
    pub dice_values: [u8; N_VALUES],
    pub roll_number: u8,
    pub scores: [Option<u16>; YahtzeeScoreCategory::variant_count()],
}

impl YahtzeeState {
    pub fn has_categories_left(&self) -> bool {
        self.scores.contains(&None)
    }

    /// Return the die number corresponding to yahtzee or None if not yahtzee.
    pub fn check_yahtzee(&self) -> Option<usize> {
        (0..N_VALUES).find(|&i| { self.dice_values[i] == N_DICE as u8 })
    }
}

impl State for YahtzeeState {
    fn get_current_player_ids(&self) -> Vec<usize> {
        return vec![0];
    }
}

impl Display for YahtzeeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - [ ", self.roll_number)?;
        for dice_value in self.dice_values {
            write!(f, "{} ", dice_value)?;
        }
        write!(f, "]\n")?;
        for i in 0..self.scores.len() {
            let score_categories = YahtzeeScoreCategory::variants();
            let category_name = &score_categories[i];
            let score = match self.scores[i] {
                Some(score) => score.to_string(),
                None => "-".to_string(),
            };
            write!(f, "{:?}: {}", category_name, score)?;
            if i != self.scores.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
