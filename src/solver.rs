use std::collections::HashSet;

use crate::operations::Operation;
use crate::problem::{check_solvable_problem, Problem, SolverResult, SolverWithAux, StateWithHistory};
use crate::state::State;

pub fn process_state_history(visited: &HashSet<State>,
                             new_states_with_history: &mut StateWithHistory,
                             new_visited: &mut HashSet<State>,
                             state: &State,
                             history: &Vec<Operation>) {
    let operations = state.available_operations();
    for op in operations {
        let new_state = state.apply(op);
        if !visited.contains(&new_state) {
            let mut new_history = history.to_owned();
            new_history.push(op);
            new_states_with_history.push((new_state.clone(), new_history));
            new_visited.insert(new_state);
        }
    }
}

pub fn solve<'a, S>(solver: &S, problem: Problem) -> SolverResult where S: SolverWithAux {
    let problem = check_solvable_problem(&problem)?;
    let start: StateWithHistory = vec![(problem.from.clone(), vec![])];
    let mut set = HashSet::new();
    set.insert(problem.from.clone());

    solver.solve_aux(problem, start, set)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::problem::{Problem, Solver};

    pub fn test_solver(input: &str,
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
}
