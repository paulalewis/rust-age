use std::fmt;

use crate::core::simulator::Action;

use super::{constants::N_VALUES, yahtzee_score_category::YahtzeeScoreCategory};

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum YahtzeeAction {
    /// The roll action controls which dice get
    /// rolled and which are kept for next state.
    /// Indicated quantity of each die number to not roll again.
    YahtzeeRollAction { selected: [u8; N_VALUES] },
    YahtzeeSelectAction { score_category: YahtzeeScoreCategory },
}

impl Action for YahtzeeAction {}

impl fmt::Display for YahtzeeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YahtzeeAction::YahtzeeRollAction { selected } => {
                write!(f, "{:?}", selected)
            }
            YahtzeeAction::YahtzeeSelectAction { score_category } => {
                write!(f, "{:?}", score_category)
            }
        }
    }
}
