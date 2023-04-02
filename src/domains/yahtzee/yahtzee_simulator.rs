use std::collections::HashSet;

use rand::{SeedableRng, RngCore};
use rand_chacha::ChaCha8Rng;

use crate::core::{simulator::{Simulator, LegalActions}, reward::{ScoreReward}};

use super::{yahtzee_state::YahtzeeState, yahtzee_action::YahtzeeAction, yahtzee_score_category::YahtzeeScoreCategory, constants::{N_VALUES, N_DICE}};

const BONUS_THRESHOLD: u16 = 63;
const BONUS_SCORE: u16 = 35;
const FULL_HOUSE_SCORE: u16 = 25;
const SMALL_STRAIGHT_SCORE: u16 = 30;
const LARGE_STRAIGHT_SCORE: u16 = 40;
const YAHTZEE_SCORE: u16 = 50;
const YAHTZEE_BONUS: u16 = 100;

pub struct YahtzeeSimulator<'a> {
    rng: &'a mut ChaCha8Rng,
}

impl <'a> YahtzeeSimulator<'a> {
    pub fn new(rng: &'a mut ChaCha8Rng) -> Self {
        YahtzeeSimulator { rng }
    }
}
 
fn roll_dice(rng: &mut ChaCha8Rng) -> [u8; N_VALUES] {
    let mut dice_values = [0; N_VALUES];
    for _ in 0..N_DICE {
        let index = (rng.next_u32() as usize) % N_VALUES;
        dice_values[index] += 1;
    }
    dice_values
}

impl <'a> Simulator<YahtzeeState, YahtzeeAction, ScoreReward> for YahtzeeSimulator<'a> {
    
    fn generate_initial_state(&mut self) -> YahtzeeState {
        YahtzeeState {
            dice_values: roll_dice(self.rng),
            roll_number: 1,
            scores: [None; YahtzeeScoreCategory::variant_count()],
        }
    }

    fn calculate_rewards(&mut self, state: &YahtzeeState) -> Vec<ScoreReward> {
        let mut score = 0u16;
        if !state.has_categories_left() {
            let scores = state.scores.iter().map(|&x| x.unwrap_or(0) as u16).collect::<Vec<u16>>();
            for i in 0..N_VALUES {
                score += scores[i];
            }
            if score >= BONUS_THRESHOLD {
                score += BONUS_SCORE;
            }
            for i in N_VALUES..scores.len() {
                score += scores[i];
            }
        }
        return vec![ScoreReward(score as isize)];
    }

    fn calculate_legal_actions(&mut self, state: &YahtzeeState) -> Vec<LegalActions<YahtzeeAction>> {
        let mut legal_actions: LegalActions<YahtzeeAction> = LegalActions(HashSet::new());
        if state.has_categories_left() {
            if state.roll_number < 3 {
                for i in 0..=state.dice_values[0] {
                    for j in 0..=state.dice_values[1] {
                        for k in 0..=state.dice_values[2] {
                            for l in 0..=state.dice_values[3] {
                                for m in 0..=state.dice_values[4] {
                                    for n in 0..=state.dice_values[5] {
                                        legal_actions.0.insert(YahtzeeAction::YahtzeeRollAction { selected: [i, j, k, l, m, n] });
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let yahtzee = state.check_yahtzee();
                if yahtzee == None || state.scores[yahtzee.unwrap()] != None {
                    (0..YahtzeeScoreCategory::variant_count())
                        .filter(|&x| state.scores[x] == None)
                        .for_each(|x| { legal_actions.0.insert(YahtzeeAction::YahtzeeSelectAction { score_category: YahtzeeScoreCategory::from_ordinal(x as i8).unwrap() }); } );
                } else {
                    legal_actions.0.insert(YahtzeeAction::YahtzeeSelectAction { score_category: YahtzeeScoreCategory::from_ordinal(yahtzee.unwrap() as i8).unwrap() });
                    if state.scores[YahtzeeScoreCategory::Yahtzee.ordinal() as usize] == None {
                        legal_actions.0.insert(YahtzeeAction::YahtzeeSelectAction { score_category: YahtzeeScoreCategory::Yahtzee });
                    }
                }
            }
        }
        return vec![legal_actions];
    }

    fn state_transition(&mut self, state: &YahtzeeState, actions: &std::collections::HashMap<usize, YahtzeeAction>) -> YahtzeeState {
        let action = &actions[&0];
        let legal_actions = self.calculate_legal_actions(state);
        if !legal_actions[0].0.contains(&action) {
            panic!("Illegal action {action}, from state {state}");
        }

        let mut dice_values = state.dice_values;
        let mut rolls = state.roll_number;
        let mut scores = state.scores;
        let yahtzee = state.check_yahtzee();
        if yahtzee != None {
            let yahtzee_score = scores[YahtzeeScoreCategory::Yahtzee.ordinal() as usize].unwrap();
            if yahtzee_score >= YAHTZEE_SCORE {
                scores[YahtzeeScoreCategory::Yahtzee.ordinal() as usize] = Some(yahtzee_score + YAHTZEE_BONUS);
            }
        }

        match action {
            YahtzeeAction::YahtzeeRollAction { selected } => {
                dice_values = *selected;
                let num_selected = dice_values.iter().sum::<u8>() as usize;
                for _ in num_selected..N_DICE {
                    let roll = self.rng.next_u32() as usize % N_DICE;
                    dice_values[roll] += 1;
                }
                rolls += 1;
            },
            YahtzeeAction::YahtzeeSelectAction { score_category } => {
                let score = match score_category {
                    YahtzeeScoreCategory::Ones => score_ones(&dice_values),
                    YahtzeeScoreCategory::Twos => score_twos(&dice_values),
                    YahtzeeScoreCategory::Threes => score_threes(&dice_values),
                    YahtzeeScoreCategory::Fours => score_fours(&dice_values),
                    YahtzeeScoreCategory::Fives => score_fives(&dice_values),
                    YahtzeeScoreCategory::Sixes => score_sixes(&dice_values),
                    YahtzeeScoreCategory::ThreeOfKind => score_three_of_a_kind(&dice_values),
                    YahtzeeScoreCategory::FourOfKind => score_four_of_a_kind(&dice_values),
                    YahtzeeScoreCategory::FullHouse => score_full_house(&dice_values), 
                    YahtzeeScoreCategory::SmallStraight => score_small_straight(&dice_values),
                    YahtzeeScoreCategory::LargeStraight => score_large_straight(&dice_values),
                    YahtzeeScoreCategory::Yahtzee => score_yahtzee(&dice_values),
                    YahtzeeScoreCategory::Chance => score_chance(&dice_values),
                };
                scores[score_category.ordinal() as usize] = Some(score);
                let rng = &mut ChaCha8Rng::from_entropy();
                dice_values = roll_dice(rng);
                rolls = 1;
            }
        }
        return YahtzeeState { dice_values: dice_values, roll_number: rolls, scores: scores };
    }
}

fn score_ones(dice_values: &[u8; N_VALUES]) -> u16 {
    dice_values[0] as u16
}
    
fn score_twos(dice_values: &[u8; N_VALUES]) -> u16 {
    dice_values[1] as u16 * 2
}

fn score_threes(dice_values: &[u8; N_VALUES]) -> u16 {
    dice_values[2] as u16 * 3
}

fn score_fours(dice_values: &[u8; N_VALUES]) -> u16 {
    dice_values[3] as u16 * 4
}

fn score_fives(dice_values: &[u8; N_VALUES]) -> u16 {
    dice_values[4] as u16 * 5
}

fn score_sixes(dice_values: &[u8; N_VALUES]) -> u16 {
    dice_values[5] as u16 * 6
}

fn score_three_of_a_kind(dice_values: &[u8; N_VALUES]) -> u16 {
    for i in 0..N_VALUES {
        if dice_values[i] >= 3 {
            return sum_of_values(dice_values);
        }
    }
    return 0;
}

fn score_four_of_a_kind(dice_values: &[u8; N_VALUES]) -> u16 {
    for i in 0..N_VALUES {
        if dice_values[i] >= 4 {
            return sum_of_values(dice_values);
        }
    }
    return 0;
}

fn score_full_house(dice_values: &[u8; N_VALUES]) -> u16 {
    let mut score = 0;
    let mut has_two = false;
    let mut has_three = false;
    for i in 0..N_VALUES {
        if dice_values[i] == 2 {
            has_two = true;
        } else if dice_values[i] == 3 {
            has_three = true;
        }
    }
    if has_two && has_three {
        score = FULL_HOUSE_SCORE;
    }
    score
}

fn score_small_straight(dice_values: &[u8; N_VALUES]) -> u16 {
    for i in 0..N_VALUES - 3 {
        if dice_values[i] > 0 && dice_values[i + 1] > 0 && dice_values[i + 2] > 0 && dice_values[i + 3] > 0 {
            return SMALL_STRAIGHT_SCORE;
        }
    }
    return 0;
}

fn score_large_straight(dice_values: &[u8; N_VALUES]) -> u16 {
    for i in 0..N_VALUES - 4 {
        if dice_values[i] > 0 && dice_values[i + 1] > 0 && dice_values[i + 2] > 0 && dice_values[i + 3] > 0 && dice_values[i + 4] > 0 {
            return LARGE_STRAIGHT_SCORE;
        }
    }
    return 0;
}

fn score_yahtzee(dice_values: &[u8; N_VALUES]) -> u16 {
    for i in 0..N_VALUES {
        if dice_values[i] == 5 {
            return YAHTZEE_SCORE;
        }
    }
    return 0;
}

fn score_chance(dice_values: &[u8; N_VALUES]) -> u16 {
    sum_of_values(dice_values)
}

fn sum_of_values(dice_values: &[u8; N_VALUES]) -> u16 {
    return dice_values.iter().sum::<u8>() as u16;
}
