use std::fmt::{Display, Error, Formatter};

use crate::solver::SolverError;
use crate::solver::SolverError::InvalidProblem;
use crate::state::State;

#[derive(Debug, Clone, PartialEq)]
pub struct Problem {
    pub from: State,
    pub to: State,
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

pub fn check_solvable(problem: &Problem) -> Result<&Problem, SolverError> {
    let from = problem.from.clone();
    let to = problem.to.clone();

    if from.glasses().len() < 2 {
        return Err(InvalidProblem {
            problem: problem.to_string(),
            reason: "Should have at least two glasses".to_string(),
        });
    }

    if from.glasses().len() != to.glasses().len() {
        return Err(InvalidProblem {
            problem: problem.to_string(),
            reason: "Should have same number of glasses".to_string(),
        });
    }

    let has_invalid_size = from
        .glasses()
        .into_iter()
        .zip(to.glasses().iter())
        .any(|(g1, g2)| g1.capacity != g2.capacity);
    if has_invalid_size {
        return Err(InvalidProblem {
            problem: problem.to_string(),
            reason: "Should have same capacity for all glasses".to_string(),
        });
    }

    Ok(problem)
}
