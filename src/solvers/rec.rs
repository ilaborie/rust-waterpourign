use std::collections::HashSet;
use std::hash::BuildHasher;

use crate::problem::{Problem, Solver, SolverResult, SolverWithAux, StateWithHistory};
use crate::problem::SolverError::UnsolvableProblem;
use crate::solver::{process_state_history, solve};
use crate::models::state::State;

#[derive(Debug)]
pub struct RecSolver();

impl SolverWithAux for RecSolver {
    fn solve_aux<S: BuildHasher>(&self, problem: &Problem, state_with_history: StateWithHistory, visited: &mut HashSet<State, S>) -> SolverResult {
        let maybe_solution = state_with_history.clone().into_iter()
            .find(|(state, _)| *state == problem.to);
        if let Some(result) = maybe_solution {
            return Ok(result.1);
        }

        let mut new_states_with_history: StateWithHistory = vec![];
        let initial_visited_size = visited.len();

        for (state, history) in state_with_history {
            process_state_history(&mut new_states_with_history, visited, &state, &history);
        }

// check visited
        if initial_visited_size == visited.len() {
            return Err(UnsolvableProblem { problem: problem.to_string() });
        }
// TailCall
        self.solve_aux(problem, new_states_with_history, visited)
    }
}

impl Solver for RecSolver {
    fn solve(&self, problem: Problem) -> SolverResult {
        solve(self, &problem)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::problem::SolverError::InvalidProblem;
    use crate::solvers::test_solver;

    use super::*;

    #[test]
    fn already_found() {
        let solver = RecSolver();
        let from = "0/2, 0/1";

        assert_eq!(test_solver(from, from, &solver), 0)
    }

    #[test]
    fn solve_1() {
        let solver = RecSolver();
        let from = "0/5, 0/3";
        let to = "4/5, 0/3";

        assert_eq!(test_solver(from, to, &solver), 7)
    }

    #[test]
    fn invalid_problem() {
        let solver = RecSolver();
        let from = State::from("0/8, 0/4, 0/2");
        let to = State::from("0/4, 0/2");
        let problem = Problem { from, to };

        let result = solver.solve(problem.clone());

        let reason = "Should have same number of glasses".to_string();
        assert_eq!(result, Err(InvalidProblem { problem: problem.to_string(), reason }))
    }

    #[test]
    fn no_solution() {
        let solver = RecSolver();
        let from = State::from("0/8, 0/4, 0/2");
        let to = State::from("1/8, 0/4, 0/2");
        let problem = Problem { from, to };

        let result = solver.solve(problem.clone());

// FIXME I just want to test the type
        assert_eq!(result, Err(UnsolvableProblem { problem: problem.to_string() }))
    }

    #[test]
    fn solve_2() {
        let solver = RecSolver();
        let from = "0/8, 0/5";
        let to = "6/8, 0/5";

        assert_eq!(test_solver(from, to, &solver), 7)
    }
}
