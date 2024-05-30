pub const WIN: isize = 1;
pub const LOSS: isize = -1;
pub const DRAW: isize = 0;

pub const ADVERSARIAL_DRAW: [isize; 2] = [DRAW, DRAW];
pub const ADVERSARIAL_P1_WIN: [isize; 2] = [WIN, LOSS];
pub const ADVERSARIAL_P1_LOSS: [isize; 2] = [LOSS, WIN];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reward_adversarial_reward_is_ordered_win_draw_loss() {
        assert!(WIN > DRAW);
        assert!(DRAW > LOSS);
        assert!(WIN > LOSS);
    }
}
