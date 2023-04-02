pub trait Reward : Clone + PartialEq + PartialOrd {}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScoreReward(pub isize);

impl Reward for ScoreReward {}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum AdversarialReward {
    Win = 1,
    Loss = -1,
    Draw = 0,
}

impl Reward for AdversarialReward {}

pub const ADVERSARIAL_DRAW: [AdversarialReward; 2] = [AdversarialReward::Draw, AdversarialReward::Draw];
pub const ADVERSARIAL_P1_WIN: [AdversarialReward; 2] = [AdversarialReward::Win, AdversarialReward::Loss];
pub const ADVERSARIAL_P1_LOSS: [AdversarialReward; 2] = [AdversarialReward::Loss, AdversarialReward::Win];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reward_adversarial_reward_is_ordered_win_draw_loss() {
        let win = AdversarialReward::Win;
        let draw = AdversarialReward::Draw;
        let loss = AdversarialReward::Loss;
        assert!(win > draw);
        assert!(draw > loss);
        assert!(win > loss);
    }
}
