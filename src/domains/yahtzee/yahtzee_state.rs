use std::fmt;

use crate::core::simulator::State;

use super::{yahtzee_score_category::YahtzeeScoreCategory, constants::{N_VALUES, N_DICE}};

#[derive(Clone, fmt::Debug, Hash, PartialEq, Eq)]
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

impl State for YahtzeeState {}

impl fmt::Display for YahtzeeState {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_categories_left_true() {
        let state = YahtzeeState {
            dice_values: [0; N_VALUES],
            roll_number: 1,
            scores: [None; YahtzeeScoreCategory::variant_count()],
        };
        assert!(state.has_categories_left());
    }

    #[test]
    fn has_categories_left_false() {
        let state = YahtzeeState {
            dice_values: [0; N_VALUES],
            roll_number: 1,
            scores: [Some(0); YahtzeeScoreCategory::variant_count()],
        };
        assert!(!state.has_categories_left());
    }

    #[test]
    fn check_yahtzee_none() {
        let state = YahtzeeState {
            dice_values: [1, 0, 1, 1, 2, 0],
            roll_number: 1,
            scores: [None; YahtzeeScoreCategory::variant_count()],
        };
        assert_eq!(state.check_yahtzee(), None);
    }

    #[test]
    fn check_yahtzee_threes() {
        let state = YahtzeeState {
            dice_values: [0, 0, 5, 0, 0, 0],
            roll_number: 1,
            scores: [None; YahtzeeScoreCategory::variant_count()],
        };
        assert_eq!(state.check_yahtzee(), Some(2));
    }

    #[test]
    fn yahtzee_state_to_string() {
        let state = YahtzeeState {
            dice_values: [0; N_VALUES],
            roll_number: 1,
            scores: [None; YahtzeeScoreCategory::variant_count()],
        };
        let expected = "1 - [ 0 0 0 0 0 0 ]\nOnes: -\nTwos: -\nThrees: -\nFours: -\nFives: -\nSixes: -\nThreeOfKind: -\nFourOfKind: -\nFullHouse: -\nSmallStraight: -\nLargeStraight: -\nYahtzee: -\nChance: -";
        
        assert_eq!(state.to_string(), expected);
    }
}
