use rust_waterpouring::solver::{RecSolver, Solver};
use rust_waterpouring::state::State;

fn main() {
    let some_problems: Vec<(&str, &str)> = vec![
        ("0/5, 0/3", "0/5, 0/3"),
        ("0/8, 0/5", "6/8, 0/5"),
        ("12/12, 0/8, 0/5", "6/12, 6/8, 0/5"),
        ("0/24, 0/13, 0/11, 0/5", "6/24, 6/13, 6/11, 0/5"),
    ];

    let (initial_state, final_state) = some_problems[0];
    let start = State::from(initial_state);
    let end = State::from(final_state);

    let solver = RecSolver();
    solve(&solver, start, end);
}

fn solve(solver: &dyn Solver, from: State, to: State) {
    println!("Solve {} -> {}", from, to);
    let result = solver.solve(from.clone(), to);
    match result {
        Err(_IncompatibleFinalState) =>
            println!("Cannot solve because initial and final states are incompatible!"),
        Err(_UnsolvableProblem) =>
            println!("No solution found!"),
        Ok(moves) => {
            println!("A solution found");
            let mut state = from;
            for m in moves {
                let next = state.apply(m);
                println!("{} with {} give {}", state, m, next);
                state = next;
            }
        }
    }
}

