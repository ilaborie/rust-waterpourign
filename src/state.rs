use std::fmt::{Display, Error, Formatter};

use crate::glass::Glass;
use crate::operation::Operation;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    glasses: Vec<Glass>
}

impl State {
    pub fn new(glasses: Vec<Glass>) -> Self {
        assert!(!glasses.is_empty(), "State require at least one Glass");
        State { glasses }
    }

    pub fn apply(&self, operation: Operation) -> State {
        unimplemented!()
    }

    pub fn available_operations(&self, operation: Operation) -> Vec<Operation> {
        unimplemented!()
    }
}


impl From<&str> for State {
    fn from(s: &str) -> Self {
        let glasses: Vec<Glass> = s.split(",").into_iter()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(Glass::from)
            .collect();
        State::new(glasses)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // FIXME use a foldLeft
        let s: Vec<String> = self.glasses.clone().into_iter()
            .map(|g| format!("{}", g))
            .collect();
        write!(f, "{}", s.join(", "))
    }
}


#[cfg(test)]
mod tests {
    use crate::glass::Glass;
    use crate::state::State;

    mod create {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn create_state() {
            let glasses = vec![Glass::empty(4)];
            let result = State::new(glasses.clone());
            assert_eq!(result, State { glasses })
        }

        #[test]
        #[should_panic]
        fn create_state_invalid() {
            let glasses: Vec<Glass> = vec![];
            let result = State::new(glasses.clone());
            assert_eq!(result, State { glasses })
        }
    }

    mod create_from_string {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn create_state_from_string() {
            let s = "4/7, 3/5, 0/2";
            let result = State::from(s);

            let glasses = vec![
                Glass::new(4, 7),
                Glass::new(3, 5),
                Glass::new(0, 2),
            ];
            assert_eq!(result, State { glasses })
        }

        #[test]
        #[should_panic]
        fn create_state_with_invalid_string() {
            let s = "4/7, 3/a, 0/2";
            State::from(s);
        }

        #[test]
        #[should_panic]
        fn create_state_with_invalid_string_2() {
            let s = "4/70/2";
            State::from(s);
        }

        #[test]
        #[should_panic]
        fn create_state_with_invalid_string_3() {
            let s = "plop";
            State::from(s);
        }
    }

    mod display {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn display_state() {
            let s = "4/7, 3/5, 0/2";
            let state = State::from(s);

            assert_eq!(s.to_owned(), format!("{}", state));
        }
    }
}
