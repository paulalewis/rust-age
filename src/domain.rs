
pub enum Domain {
    Connect4,
    Yahtzee,
}

impl Domain {
    /*
    pub fn get_simulator(&self) -> Box<dyn Simulator> {
        match self {
            Domain::Connect4 => Box::new(connect4::Connect4Simulator::new()),
            Domain::Yahtzee => Box::new(yahtzee::YahtzeeSimulator::new()),
        }
    }
    */
}
