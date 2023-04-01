#[derive(Clone, PartialEq, PartialOrd)]
pub enum Reward {
    Adversarial(AdversarialReward),
    Score(isize),
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum AdversarialReward {
    Win = 1,
    Loss = -1,
    Draw = 0,
}

pub fn get_adversarial_draw() -> Vec<Reward> {
    vec![Reward::Adversarial(AdversarialReward::Draw), Reward::Adversarial(AdversarialReward::Draw)]
}

pub fn get_adversarial_p1_win() -> Vec<Reward> {
    vec![Reward::Adversarial(AdversarialReward::Win), Reward::Adversarial(AdversarialReward::Loss)]
}

pub fn get_adversarial_p1_loss () -> Vec<Reward> {
    vec![Reward::Adversarial(AdversarialReward::Loss), Reward::Adversarial(AdversarialReward::Win)]
}
