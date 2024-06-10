pub const WIN: Reward = Reward(1);
pub const LOSS: Reward = Reward(-1);
pub const DRAW: Reward = Reward(0);

pub const ADVERSARIAL_DRAW: [Reward; 2] = [DRAW, DRAW];
pub const ADVERSARIAL_P1_WIN: [Reward; 2] = [WIN, LOSS];
pub const ADVERSARIAL_P1_LOSS: [Reward; 2] = [LOSS, WIN];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reward(pub isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reward_adversarial_reward_is_ordered_win_draw_loss() {
        assert!(WIN.0 > DRAW.0);
        assert!(DRAW.0 > LOSS.0);
        assert!(WIN.0 > LOSS.0);
    }
}
