use std::env;

use rust_waterpouring::problem::{Problem, Solver};
use rust_waterpouring::problem::SolverError::{InvalidProblem, UnsolvableProblem};
use rust_waterpouring::solvers::imp::ImperativeSolver;
use rust_waterpouring::solvers::rec2::Rec2Solver;
use rust_waterpouring::solvers::rec::RecSolver;
use rust_waterpouring::state::State;

fn main() {
    let some_problems: Vec<(&str, &str)> = vec![
        ("0/5, 0/3", "4/5, 0/3"),
        ("0/8, 0/5", "6/8, 0/5"),
        ("12/12, 0/8, 0/5", "6/12, 6/8, 0/5"),
        ("0/24, 0/13, 0/11, 0/5", "6/24, 6/13, 6/11, 0/5"),
    ];

    let solver_ref = env::args()
        .nth(1)
        .unwrap();

    let index: usize = env::args()
        .nth(2)
        .map_or(0, |s| s.parse::<usize>().unwrap_or(0));


    let (initial_state, final_state) = some_problems[index];
    let start = State::from(initial_state);
    let end = State::from(final_state);

    match solver_ref.as_str() {
        "rec" => solve(&RecSolver(), &start, end),
        "rec2" => solve(&Rec2Solver(), &start, end),
        "imp" => solve(&ImperativeSolver(), &start, end),
        _ => panic!("Solver not found, use 'rec', 'rec2', 'imp'")
    };
}

// FIXME May change Solver to function
fn solve<S>(solver: &S, from: &State, to: State) where S: Solver {
    println!("Solve {} -> {}", from, to);
    let problem = Problem::new(from.clone(), to);
    let result = solver.solve(problem.clone());
    match result {
        Err(InvalidProblem { reason, .. }) =>
            println!("Cannot solve because {}", reason),
        Err(UnsolvableProblem { .. }) =>
            println!("No solution found!"),
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

