use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

use waterpouring_model::operations::Operation;
use waterpouring_model::problem::Problem;
use waterpouring_model::solver::SolverError::{InvalidProblem, UnsolvableProblem};
use waterpouring_model::solver::{Solver, SolverError};
use waterpouring_model::state::State;
use waterpouring_rec::rec::RecSolver;

use crate::result::WasmResult;
use crate::step::WasmStep;

mod operation;
mod result;
mod step;

//When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
//allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn solve(from: &str, to: &str) -> JsValue {
    let problem = Problem::from((from, to));
    let solver = RecSolver();

    let result = transform(&problem.from.clone(), solver.solve(problem));

    JsValue::from_serde(&result).unwrap()
}

fn transform(from: &State, result: Result<Vec<Operation>, SolverError>) -> WasmResult {
    match result {
        Err(InvalidProblem { reason, .. }) => {
            let err = format!("Cannot solve because {}", reason);
            WasmResult::error(err)
        }
        Err(UnsolvableProblem { .. }) => WasmResult::error("No solution found!".into()),
        Ok(moves) => {
            let mut steps: Vec<WasmStep> = vec![];
            steps.push(WasmStep::init(from));
            let mut state = from.clone();
            for m in moves {
                let (step, next) = WasmStep::step(&state, &m);
                steps.push(step);
                state = next;
            }
            WasmResult::solved(steps)
        }
    }
}
