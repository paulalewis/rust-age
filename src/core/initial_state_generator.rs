use super::simulator::State;

pub trait InitialStateGenerator {
    type S : State;
    
    /// Generates an initial state for the domain.
    /// 
    /// The initial state returned is not necessarily always
    /// the same state.
    /// 
    /// ### Return Value
    /// 
    /// Returns an inital state in the domain.
    fn generate_initial_state(&self) -> Self::S;
}