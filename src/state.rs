use std::collections::HashSet;

use crate::glass::Glass;
use crate::operation::Operation;

#[derive(Debug, Eq, PartialEq)]
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

    pub fn available_operations(&self, operation: Operation) -> HashSet<Operation> {
        unimplemented!()
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
}
