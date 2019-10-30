use std::collections::HashSet;

use crate::problem::{check_solvable_problem, Problem, Solver, SolverResult, StateWithHistory};
use crate::problem::SolverError::UnsolvableProblem;
use crate::solver::process_state_history;
use crate::state::State;

#[derive(Debug)]
pub struct ImperativeSolver();

impl Solver for ImperativeSolver {
    fn solve(&self, problem: Problem) -> SolverResult {
// Check
        let problem = check_solvable_problem(&problem)?;

// first iteration
        let mut states_with_history: StateWithHistory = vec![(problem.from.clone(), vec![])];
        let mut visited: HashSet<State> = HashSet::new();
        visited.insert(problem.from.clone());

        loop {
            let maybe_solution = states_with_history.clone().into_iter()
                .find(|(state, _)| *state == problem.to.clone());
            if let Some(result) = maybe_solution {
                return Ok(result.1);
            }

// Build new solution
            let mut new_states_with_history: StateWithHistory = vec![];
            let mut new_visited: HashSet<State> = visited.clone();

            for (state, history) in states_with_history {
                process_state_history(&visited, &mut new_states_with_history, &mut new_visited, &state, &history);
            }

// check visited
            if new_visited.len() == visited.len() {
                return Err(UnsolvableProblem { problem: problem.to_string() });
            }

            states_with_history = new_states_with_history;
            visited = new_visited;
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::problem::SolverError::InvalidProblem;

    use super::*;
    use crate::solvers::test_solver;

    #[test]
    fn already_found() {
        let solver = ImperativeSolver();
        let from = "0/2, 0/1";

        assert_eq!(test_solver(from, from, &solver), 0)
    }

    #[test]
    fn solve_1() {
        let solver = ImperativeSolver();
        let from = "0/5, 0/3";
        let to = "4/5, 0/3";

        assert_eq!(test_solver(from, to, &solver), 7)
    }

    #[test]
    fn invalid_problem() {
        let solver = ImperativeSolver();
        let from = State::from("0/8, 0/4, 0/2");
        let to = State::from("0/4, 0/2");
        let problem = Problem { from, to };

        let result = solver.solve(problem.clone());

        let reason = "Should have same number of glasses".to_string();
        assert_eq!(result, Err(InvalidProblem { problem: problem.to_string(), reason }))
    }

    #[test]
    fn no_solution() {
        let solver = ImperativeSolver();
        let from = State::from("0/8, 0/4, 0/2");
        let to = State::from("1/8, 0/4, 0/2");
        let problem = Problem { from, to };

        let result = solver.solve(problem.clone());

// FIXME I just want to test the type
        assert_eq!(result, Err(UnsolvableProblem { problem: problem.to_string() }))
    }

    #[test]
    fn solve_2() {
        let solver = ImperativeSolver();
        let from = "0/8, 0/5";
        let to = "6/8, 0/5";

        assert_eq!(test_solver(from, to, &solver), 7)
    }
}
