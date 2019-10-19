use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};

use crate::operation::Operation;
use crate::solver::SolverError::{InvalidProblem, UnsolvableProblem};
use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct Problem {
    from: State,
    to: State,
}

impl Problem {
    pub fn new(from: State, to: State) -> Self {
        Self { from, to }
    }
}

impl From<(&str, &str)> for Problem {
    fn from(pair: (&str, &str)) -> Self {
        Self {
            from: State::from(pair.0),
            to: State::from(pair.1),
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} -> {}", self.from.clone(), self.to.clone())
    }
}

#[derive(Debug, PartialEq)]
pub enum SolverError {
    InvalidProblem { problem: Problem, reason: String },
    UnsolvableProblem { problem: Problem },
}

pub type SolverResult = Result<Vec<Operation>, SolverError>;

// FIXME maybe of just `fn`
pub type ASolver = dyn Fn(State, State) -> SolverResult;

pub trait Solver {
    fn solve(&self, problem: Problem) -> SolverResult;
}

pub trait SolverWithAux {
    fn solve_aux(&self, problem: Problem, state_with_history: StateWithHistory, visited: HashSet<State>) -> SolverResult;
}

type History = Vec<Operation>;

type StateWithHistory = Vec<(State, History)>;

fn check_solvable_problem(problem: Problem) -> Option<SolverError> {
    let from = problem.from.clone();
    let to = problem.to.clone();

    if from.glasses().len() < 2 {
        return Some(InvalidProblem { problem: problem.clone(), reason: "Should have at least two glasses".to_string() });
    }

    if from.glasses().len() != to.glasses().len() {
        return Some(InvalidProblem { problem: problem.clone(), reason: "Should have same number of glasses".to_string() });
    }

    let has_invalid_size = from.glasses().into_iter()
        .zip(to.glasses().iter())
        .any(|(g1, g2)| g1.capacity != g2.capacity);
    if has_invalid_size {
        return Some(InvalidProblem { problem: problem.clone(), reason: "Should have same capacity for all glasses".to_string() });
    }

    None
}

fn process_state_history(visited: &HashSet<State>,
                         new_states_with_history: &mut StateWithHistory,
                         new_visited: &mut HashSet<State>,
                         state: State,
                         history: History) {
    let operations = state.available_operations();
    for op in operations {
        let new_state = state.apply(op);
        if !visited.contains(&new_state) {
            let mut new_history = history.clone();
            new_history.push(op);
            new_states_with_history.push((new_state.clone(), new_history));
            new_visited.insert(new_state);
        }
    }
}

fn solve<S>(solver: &S, problem: Problem) -> SolverResult where S: SolverWithAux {
    let check = check_solvable_problem(problem.clone());
    if let Some(msg) = check {
        return Err(msg);
    }

    let start: StateWithHistory = vec![(problem.from.clone(), vec![])];
    let mut set = HashSet::new();
    set.insert(problem.from.clone());
    solver.solve_aux(problem, start, set)
}

// Rec
#[derive(Debug)]
pub struct RecSolver();

impl SolverWithAux for RecSolver {
    fn solve_aux(&self, problem: Problem, state_with_history: StateWithHistory, visited: HashSet<State>) -> SolverResult {
        let maybe_solution = state_with_history.clone().into_iter()
            .find(|(state, _)| *state == problem.to);
        if let Some(result) = maybe_solution {
            return Ok(result.1);
        }

        let mut new_states_with_history: StateWithHistory = vec![];
        let mut new_visited: HashSet<State> = visited.clone();

        for (state, history) in state_with_history {
            process_state_history(&visited, &mut new_states_with_history, &mut new_visited, state, history);
        }

// check visited
        if new_visited.len() == visited.len() {
            return Err(UnsolvableProblem { problem });
        }
// TailCall
        self.solve_aux(problem, new_states_with_history, new_visited)
    }
}

impl Solver for RecSolver {
    fn solve(&self, problem: Problem) -> SolverResult {
        solve(self, problem)
    }
}


// More imperative

#[derive(Debug)]
pub struct Rec2Solver();

impl SolverWithAux for Rec2Solver {
    fn solve_aux(&self, problem: Problem, state_with_history: StateWithHistory, visited: HashSet<State>) -> SolverResult {
        let mut new_states_with_history: StateWithHistory = vec![];
        let mut new_visited: HashSet<State> = visited.clone();

        for (state, history) in state_with_history {
            if state == problem.to {
                return Ok(history);
            }
            process_state_history(&visited, &mut new_states_with_history, &mut new_visited, state, history);
        }

// check visited
        if new_visited.len() == visited.len() {
            return Err(UnsolvableProblem { problem });
        }
// TailCall
        self.solve_aux(problem, new_states_with_history, new_visited)
    }
}

impl Solver for Rec2Solver {
    fn solve(&self, problem: Problem) -> SolverResult {
        solve(self, problem)
    }
}

// More imperative

#[derive(Debug)]
pub struct ImperativeSolver();

impl Solver for ImperativeSolver {
    fn solve(&self, problem: Problem) -> SolverResult {
// Check
        let check = check_solvable_problem(problem.clone());
        if let Some(result) = check {
            return Err(result);
        }

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
                process_state_history(&visited, &mut new_states_with_history, &mut new_visited, state, history);
            }

// check visited
            if new_visited.len() == visited.len() {
                return Err(UnsolvableProblem { problem });
            }

            states_with_history = new_states_with_history;
            visited = new_visited;
        }
    }
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    fn test_solver(input: &str,
                   output: &str,
                   solver: &dyn Solver,
                   expected_size: usize) {
        let from = State::from(input);
        let to = State::from(output);
        let problem = Problem { from, to };

        let result = solver.solve(problem.clone());

        let size = result.map(|lst| lst.len())
            .expect("Should found a solution");
        assert_eq!(size, expected_size)
    }

    mod rec {
        use pretty_assertions::assert_eq;

        use crate::solver::SolverError::{InvalidProblem, UnsolvableProblem};

        use super::*;

        #[test]
        fn already_found() {
            let solver = RecSolver();
            let from = "0/2, 0/1";

            test_solver(from, from, &solver, 0)
        }

        #[test]
        fn solve_1() {
            let solver = RecSolver();
            let from = "0/5, 0/3";
            let to = "4/5, 0/3";

            test_solver(from, to, &solver, 7)
        }

        #[test]
        fn invalid_problem() {
            let solver = RecSolver();
            let from = State::from("0/8, 0/4, 0/2");
            let to = State::from("0/4, 0/2");
            let problem = Problem { from, to };

            let result = solver.solve(problem.clone());

            let reason = "Should have same number of glasses".to_string();
            assert_eq!(result, Err(InvalidProblem { problem, reason }))
        }

        #[test]
        fn no_solution() {
            let solver = RecSolver();
            let from = State::from("0/8, 0/4, 0/2");
            let to = State::from("1/8, 0/4, 0/2");
            let problem = Problem { from, to };

            let result = solver.solve(problem.clone());

// FIXME I just want to test the type
            assert_eq!(result, Err(UnsolvableProblem { problem }))
        }

        #[test]
        fn solve_2() {
            let solver = RecSolver();
            let from = "0/8, 0/5";
            let to = "6/8, 0/5";

            test_solver(from, to, &solver, 7)
        }
    }

    mod rec2 {
        use pretty_assertions::assert_eq;

        use crate::solver::SolverError::{InvalidProblem, UnsolvableProblem};

        use super::*;

        #[test]
        fn already_found() {
            let solver = Rec2Solver();
            let from = "0/2, 0/1";

            test_solver(from, from, &solver, 0)
        }

        #[test]
        fn solve_1() {
            let solver = Rec2Solver();
            let from = "0/5, 0/3";
            let to = "4/5, 0/3";

            test_solver(from, to, &solver, 7)
        }

        #[test]
        fn invalid_problem() {
            let solver = Rec2Solver();
            let from = State::from("0/8, 0/4, 0/2");
            let to = State::from("0/4, 0/2");
            let problem = Problem { from, to };

            let result = solver.solve(problem.clone());

            let reason = "Should have same number of glasses".to_string();
            assert_eq!(result, Err(InvalidProblem { problem, reason }))
        }

        #[test]
        fn no_solution() {
            let solver = Rec2Solver();
            let from = State::from("0/8, 0/4, 0/2");
            let to = State::from("1/8, 0/4, 0/2");
            let problem = Problem { from, to };

            let result = solver.solve(problem.clone());

// FIXME I just want to test the type
            assert_eq!(result, Err(UnsolvableProblem { problem }))
        }

        #[test]
        fn solve_2() {
            let solver = Rec2Solver();
            let from = "0/8, 0/5";
            let to = "6/8, 0/5";

            test_solver(from, to, &solver, 7)
        }
    }

    mod imperative {
        use pretty_assertions::assert_eq;

        use crate::solver::SolverError::{InvalidProblem, UnsolvableProblem};

        use super::*;

        #[test]
        fn already_found() {
            let solver = ImperativeSolver();
            let from = "0/2, 0/1";

            test_solver(from, from, &solver, 0)
        }

        #[test]
        fn solve_1() {
            let solver = ImperativeSolver();
            let from = "0/5, 0/3";
            let to = "4/5, 0/3";

            test_solver(from, to, &solver, 7)
        }

        #[test]
        fn invalid_problem() {
            let solver = ImperativeSolver();
            let from = State::from("0/8, 0/4, 0/2");
            let to = State::from("0/4, 0/2");
            let problem = Problem { from, to };

            let result = solver.solve(problem.clone());

            let reason = "Should have same number of glasses".to_string();
            assert_eq!(result, Err(InvalidProblem { problem, reason }))
        }

        #[test]
        fn no_solution() {
            let solver = ImperativeSolver();
            let from = State::from("0/8, 0/4, 0/2");
            let to = State::from("1/8, 0/4, 0/2");
            let problem = Problem { from, to };

            let result = solver.solve(problem.clone());

// FIXME I just want to test the type
            assert_eq!(result, Err(UnsolvableProblem { problem }))
        }

        #[test]
        fn solve_2() {
            let solver = ImperativeSolver();
            let from = "0/8, 0/5";
            let to = "6/8, 0/5";

            test_solver(from, to, &solver, 7)
        }
    }
}
