use std::collections::HashSet;
use std::hash::BuildHasher;

use waterpouring_model::operations::Operation;
use waterpouring_model::problem::{check_solvable, Problem};
use waterpouring_model::solver::SolverError::UnsolvableProblem;
use waterpouring_model::solver::{Solver, SolverResult, StateWithHistory};
use waterpouring_model::state::State;

#[derive(Debug)]
pub struct ImperativeSolver();

impl ImperativeSolver {
    fn process_state_history<S: BuildHasher>(
        new_states_with_history: &mut StateWithHistory,
        visited: &mut HashSet<State, S>,
        state: &State,
        history: &[Operation],
    ) {
        let operations = state.available_operations();
        for op in operations {
            let new_state = state.apply(op);
            if !visited.contains(&new_state) {
                let mut new_history = history.to_owned();
                new_history.push(op);
                new_states_with_history.push((new_state.clone(), new_history));
                visited.insert(new_state);
            }
        }
    }
}

impl Solver for ImperativeSolver {
    fn solve(&self, problem: Problem) -> SolverResult {
        // Check
        let problem = check_solvable(&problem)?;

        // first iteration
        let mut states_with_history: StateWithHistory = vec![(problem.from.clone(), vec![])];
        let mut visited: HashSet<State> = HashSet::new();
        visited.insert(problem.from.clone());

        loop {
            let maybe_solution = states_with_history
                .clone()
                .into_iter()
                .find(|(state, _)| *state == problem.to.clone());
            if let Some(result) = maybe_solution {
                return Ok(result.1);
            }

            // find next states
            let mut new_states_with_history: StateWithHistory = vec![];
            let initial_visited_size = visited.len();

            for (state, history) in states_with_history {
                Self::process_state_history(
                    &mut new_states_with_history,
                    &mut visited,
                    &state,
                    &history,
                );
            }

            // check visited
            if initial_visited_size == visited.len() {
                return Err(UnsolvableProblem {
                    problem: problem.to_string(),
                });
            }

            states_with_history = new_states_with_history;
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use waterpouring_model::solver::test_solver;
    use waterpouring_model::solver::SolverError::InvalidProblem;

    use super::*;

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
        assert_eq!(
            result,
            Err(InvalidProblem {
                problem: problem.to_string(),
                reason,
            })
        )
    }

    #[test]
    fn no_solution() {
        let solver = ImperativeSolver();
        let from = State::from("0/8, 0/4, 0/2");
        let to = State::from("1/8, 0/4, 0/2");
        let problem = Problem { from, to };

        let result = solver.solve(problem.clone());

        // FIXME I just want to test the type
        assert_eq!(
            result,
            Err(UnsolvableProblem {
                problem: problem.to_string()
            })
        )
    }

    #[test]
    fn solve_2() {
        let solver = ImperativeSolver();
        let from = "0/8, 0/5";
        let to = "6/8, 0/5";

        assert_eq!(test_solver(from, to, &solver), 7)
    }
}
