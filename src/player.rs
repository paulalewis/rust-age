use crate::agent::{RandomAgent, IoAgent};

pub trait Player {
    fn get_id(&self) -> usize;
}

pub struct HumanPlayer {
    id: usize,
    pub agent: IoAgent,
}

impl HumanPlayer {
    pub fn new(id: usize, agent: IoAgent) -> Self {
        HumanPlayer {
            id,
            agent,
        }
    }
}

impl Player for HumanPlayer {
    fn get_id(&self) -> usize {
        self.id.clone()
    }
}

pub struct RandomPlayer {
    id: usize,
    pub agent: RandomAgent,
}

impl RandomPlayer {
    pub fn new(id: usize, agent: RandomAgent) -> Self {
        RandomPlayer {
            id,
            agent,
        }
    }
}

impl Player for RandomPlayer {
    fn get_id(&self) -> usize {
        self.id
    }
}
