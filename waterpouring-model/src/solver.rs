use crate::operations::Operation;
use crate::problem::Problem;
use crate::solver::SolverError::{InvalidProblem, UnsolvableProblem};
use crate::state::State;

#[derive(Debug, PartialEq)]
pub enum SolverError {
    InvalidProblem { problem: String, reason: String },
    UnsolvableProblem { problem: String },
}

pub type SolverResult = Result<Vec<Operation>, SolverError>;

// FIXME maybe of just `fn`
//pub type ASolver = dyn Fn(State, State) -> SolverResult;

pub trait Solver {
    fn solve(&self, problem: Problem) -> SolverResult;
}

pub type StateWithHistory = Vec<(State, Vec<Operation>)>;

pub fn test_solver(input: &str, output: &str, solver: &dyn Solver) -> usize {
    let from = State::from(input);
    let to = State::from(output);
    let problem = Problem { from, to };

    solver
        .solve(problem.clone())
        .map(|lst| lst.len())
        .expect("Should found a solution")
}

pub fn solve<S>(solver: &S, from: &State, to: State)
where
    S: Solver,
{
    println!("Solve {} -> {}", from, to);
    let problem = Problem::new(from.clone(), to);
    let result = solver.solve(problem.clone());
    match result {
        Err(InvalidProblem { reason, .. }) => println!("Cannot solve because {}", reason),
        Err(UnsolvableProblem { .. }) => println!("No solution found!"),
        Ok(moves) => {
            println!("A solution found");
            let mut state = from.clone();
            for m in moves {
                let next = state.apply(m);
                println!("{} with {} give {}", state, m, next);
                state = next;
            }
        }
    }
}
