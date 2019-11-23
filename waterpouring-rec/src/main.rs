use std::env;

use waterpouring_model::solver::solve;
use waterpouring_model::state::State;
use waterpouring_rec::rec::RecSolver;

fn main() {
    let some_problems: Vec<(&str, &str)> = vec![
        ("0/5, 0/3", "4/5, 0/3"),
        ("0/8, 0/5", "6/8, 0/5"),
        ("12/12, 0/8, 0/5", "6/12, 6/8, 0/5"),
        ("0/24, 0/13, 0/11, 0/5", "6/24, 6/13, 6/11, 0/5"),
    ];

    let index: usize = env::args()
        .nth(1)
        .map_or(0, |s| s.parse::<usize>().unwrap_or(0));

    let (initial_state, final_state) = some_problems[index];
    let start = State::from(initial_state);
    let end = State::from(final_state);

    solve(&RecSolver(), &start, end)
}
