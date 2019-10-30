use std::collections::HashSet;

use crate::problem::{Problem, Solver, SolverResult, SolverWithAux, StateWithHistory};
use crate::problem::SolverError::UnsolvableProblem;
use crate::solver::{process_state_history, solve};
use crate::state::State;

#[derive(Debug)]
pub struct Rec2Solver();

impl SolverWithAux for Rec2Solver {
    fn solve_aux(&self, problem: &Problem, state_with_history: StateWithHistory, visited: HashSet<State>) -> SolverResult {
        let mut new_states_with_history: StateWithHistory = vec![];
        let mut new_visited: HashSet<State> = visited.clone();

        for (state, history) in state_with_history {
            if state == problem.to {
                return Ok(history);
            }
            process_state_history(&visited, &mut new_states_with_history, &mut new_visited, &state, &history);
        }

// check visited
        if new_visited.len() == visited.len() {
            return Err(UnsolvableProblem { problem: problem.to_string() });
        }
// TailCall
        self.solve_aux(problem, new_states_with_history, new_visited)
    }
}

impl Solver for Rec2Solver {
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
        let solver = Rec2Solver();
        let from = "0/2, 0/1";

        assert_eq!(test_solver(from, from, &solver), 0)
    }

    #[test]
    fn solve_1() {
        let solver = Rec2Solver();
        let from = "0/5, 0/3";
        let to = "4/5, 0/3";

        assert_eq!(test_solver(from, to, &solver), 7)
    }

    #[test]
    fn invalid_problem() {
        let solver = Rec2Solver();
        let from = State::from("0/8, 0/4, 0/2");
        let to = State::from("0/4, 0/2");
        let problem = Problem { from, to };

        let result = solver.solve(problem.clone());

        let reason = "Should have same number of glasses".to_string();
        assert_eq!(result, Err(InvalidProblem { problem: problem.to_string(), reason }))
    }

    #[test]
    fn no_solution() {
        let solver = Rec2Solver();
        let from = State::from("0/8, 0/4, 0/2");
        let to = State::from("1/8, 0/4, 0/2");
        let problem = Problem { from, to };

        let result = solver.solve(problem.clone());

// FIXME I just want to test the type
        assert_eq!(result, Err(UnsolvableProblem { problem: problem.to_string() }))
    }

    #[test]
    fn solve_2() {
        let solver = Rec2Solver();
        let from = "0/8, 0/5";
        let to = "6/8, 0/5";

        assert_eq!(test_solver(from, to, &solver), 7)
    }
}
