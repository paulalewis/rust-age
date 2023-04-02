use std::fmt;

use crate::core::simulator::Action;

use super::{constants::N_VALUES, yahtzee_score_category::YahtzeeScoreCategory};

#[derive(Clone, fmt::Debug, Hash, PartialEq, Eq)]
pub enum YahtzeeAction {

    /// Select dice to hold for next roll.
    SelectDice { selected: [u8; N_VALUES] },

    /// Select a category to score.
    SelectCategory { score_category: YahtzeeScoreCategory },
}

impl Action for YahtzeeAction {}

impl fmt::Display for YahtzeeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YahtzeeAction::SelectDice { selected } => {
                write!(f, "{:?}", selected)
            }
            YahtzeeAction::SelectCategory { score_category } => {
                write!(f, "{:?}", score_category)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yahtzee_action_select_dice_to_string() {
        let action = YahtzeeAction::SelectDice { selected: [0; N_VALUES] };
        assert_eq!(action.to_string(), "[0, 0, 0, 0, 0, 0]");
    }

    #[test]
    fn yahtzee_action_select_category_to_string() {
        let action = YahtzeeAction::SelectCategory {
            score_category: YahtzeeScoreCategory::Ones,
        };
        assert_eq!(action.to_string(), "Ones");
    }
}
