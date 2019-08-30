use crate::operation::Operation;
use crate::state::State;

pub enum SolverError {
    InvalidFinalState { from: State, to: State },
    UnsolvableProblem { from: State, to: State },
}

pub type SolverResult = Result<Vec<Operation>, SolverError>;

pub trait Solver {
    fn solve(&self, from: State, to: State) -> Result<Vec<Operation>, SolverError>;
}

// Rec
#[derive(Debug)]
pub struct RecSolver();

impl Solver for RecSolver {
    fn solve(&self, from: State, to: State) -> SolverResult {
        unimplemented!()
    }
}

//pub const REC_SOLVER: Fn<(State, State), SolverResult> = RecSolver().solve;


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
}
