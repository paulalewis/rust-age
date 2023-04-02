pub trait Reward : Clone + PartialEq + PartialOrd {}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct ScoreReward(pub isize);

impl Reward for ScoreReward {}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum AdversarialReward {
    Win = 1,
    Loss = -1,
    Draw = 0,
}

impl Reward for AdversarialReward {}

pub fn get_adversarial_draw() -> Vec<AdversarialReward> {
    vec![AdversarialReward::Draw, AdversarialReward::Draw]
}

pub fn get_adversarial_p1_win() -> Vec<AdversarialReward> {
    vec![AdversarialReward::Win, AdversarialReward::Loss]
}

pub fn get_adversarial_p1_loss () -> Vec<AdversarialReward> {
    vec![AdversarialReward::Loss, AdversarialReward::Win]
}
