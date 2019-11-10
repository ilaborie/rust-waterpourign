use crate::problem::{Problem, Solver};
use crate::models::state::State;

pub mod imp;
pub mod rec;
pub mod rec2;

pub fn test_solver(input: &str, output: &str, solver: &dyn Solver) -> usize {
    let from = State::from(input);
    let to = State::from(output);
    let problem = Problem { from, to };

    solver.solve(problem.clone())
        .map(|lst| lst.len())
        .expect("Should found a solution")
}
