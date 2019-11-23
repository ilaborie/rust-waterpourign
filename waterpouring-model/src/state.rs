use std::fmt::{Display, Error, Formatter};
use std::hash::Hash;

use crate::glass::Glass;
use crate::operations::Operation;
use crate::operations::Operation::{Empty, Fill, Pour};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    glasses: Vec<Glass>,
}

impl State {
    pub fn new(glasses: Vec<Glass>) -> Self {
        assert!(!glasses.is_empty(), "State require at least one Glass");
        Self { glasses }
    }

    pub fn glasses(&self) -> Vec<Glass> {
        self.glasses.clone()
    }

    pub fn apply(&self, operation: Operation) -> Self {
        let next_glasses = self
            .glasses
            .iter()
            .enumerate()
            .map(|(idx, g)| match operation {
                Empty { glass } => {
                    if idx == glass {
                        g.empty()
                    } else {
                        g.clone()
                    }
                }
                Fill { glass } => {
                    if idx == glass {
                        g.fill()
                    } else {
                        g.clone()
                    }
                }
                Pour { from, to } => {
                    if idx == from {
                        g.sub(self.glasses[to].remaining_capacity())
                    }
                    //
                    else if idx == to {
                        g.add(self.glasses[from].current)
                    }
                    //
                    else {
                        g.clone()
                    }
                }
            })
            .collect();

        Self::new(next_glasses)
    }

    pub fn available_operations(&self) -> Vec<Operation> {
        // FIXME should try over ways to write this
        let mut result: Vec<Operation> = vec![];

        for (index, glass) in self.glasses.iter().enumerate() {
            if !glass.is_full() {
                result.push(Operation::fill(index))
            }
            if !glass.is_empty() {
                result.push(Operation::empty(index));
                // pouring
                for (dest_index, dest_glass) in self.glasses.iter().enumerate() {
                    if !dest_glass.is_full() && index != dest_index {
                        result.push(Operation::pour(index, dest_index));
                    }
                }
            }
        }

        result
    }
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let glasses: Vec<Glass> = s
            .split(',')
            .map(str::trim)
            .filter_map(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(Glass::from(s))
                }
            })
            .collect();
        Self::new(glasses)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        // FIXME use a foldLeft
        let s: Vec<String> = self
            .glasses
            .clone()
            .into_iter()
            .map(|g| format!("{}", g))
            .collect();
        write!(f, "{}", s.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::glass::Glass;
    use crate::operations::Operation;

    mod create {

        use super::*;

        #[test]
        fn create_state() {
            let glasses = vec![Glass::new_empty(4)];
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

            let glasses = vec![Glass::new(4, 7), Glass::new(3, 5), Glass::new(0, 2)];
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

    mod available_operations {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn one_fill_glass() {
            let state = State::from("10/10");

            let result = state.available_operations();

            let expected: Vec<Operation> = vec![Operation::empty(0)];
            assert_eq!(&result[..], &expected[..])
        }

        #[test]
        fn one_empty_glass() {
            let state = State::from("0/10");

            let result = state.available_operations();

            let expected: Vec<Operation> = vec![Operation::fill(0)];
            assert_eq!(&result[..], &expected[..])
        }

        #[test]
        fn one_glass() {
            let state = State::from("4/10");

            let result = state.available_operations();

            let expected: Vec<Operation> = vec![Operation::fill(0), Operation::empty(0)];
            assert_eq!(&result[..], &expected[..])
        }

        #[test]
        fn two_glass() {
            let state = State::from("5/10, 1/5");

            let result = state.available_operations();

            // FIXME custom matchers ?
            // how test contains all without the order
            let expected: Vec<Operation> = vec![
                Operation::fill(0),
                Operation::empty(0),
                Operation::pour(0, 1),
                Operation::fill(1),
                Operation::empty(1),
                Operation::pour(1, 0),
            ];
            assert_eq!(&result[..], &expected[..])
        }
    }

    mod apply {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn empty_first_glass() {
            let state = State::from("4/5, 1/3, 0/2");
            let op = Operation::empty(0);

            let result = state.apply(op);
            let end = format!("{}", result);

            assert_eq!(end, "0/5, 1/3, 0/2")
        }

        #[test]
        fn empty_second_glass() {
            let state = State::from("4/5, 1/3, 0/2");
            let op = Operation::fill(1);

            let result = state.apply(op);
            let end = format!("{}", result);

            assert_eq!(end, "4/5, 3/3, 0/2")
        }

        #[test]
        fn pour_first_to_last() {
            let state = State::from("4/5, 1/3, 0/2");
            let op = Operation::pour(0, 2);

            let result = state.apply(op);
            let end = format!("{}", result);

            assert_eq!(end, "2/5, 1/3, 2/2")
        }
    }
}
