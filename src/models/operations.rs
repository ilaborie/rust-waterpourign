use std::fmt::{Debug, Display, Error, Formatter};

use crate::models::operations::Operation::{Empty, Fill, Pour};

pub type GlassId = usize;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Operation {
    Empty { glass: GlassId },
    Fill { glass: GlassId },
    Pour { from: GlassId, to: GlassId },
}

impl Operation {
    pub fn empty(glass: GlassId) -> Self {
        Empty { glass }
    }
    pub fn fill(glass: GlassId) -> Self {
        Fill { glass }
    }
    pub fn pour(from: GlassId, to: GlassId) -> Self {
        assert_ne!(from, to, "Cannot pour an glass into itself");
        Pour { from, to }
    }

    fn as_string(&self) -> String {
        match self {
            Empty { glass } => format!("Empty({})", glass),
            Fill { glass } => format!("Fill({})", glass),
            Pour { from, to } => format!("Pour({}->{})", from, to),
        }
    }
}

impl Debug for Operation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_string())
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn create_empty() {
        let glass: GlassId = 0;

        let result = Operation::empty(glass);

        assert_eq!(result, Empty { glass })
    }

    #[test]
    fn display_empty() {
        let glass: GlassId = 0;

        let op = Operation::empty(glass);

        assert_eq!("Empty(0)".to_owned(), format!("{}", op))
    }

    #[test]
    fn create_fill() {
        let glass: GlassId = 0;

        let result = Operation::fill(glass);

        assert_eq!(result, Fill { glass })
    }

    #[test]
    fn display_fill() {
        let glass: GlassId = 0;

        let op = Operation::fill(glass);

        assert_eq!("Fill(0)".to_owned(), format!("{}", op))
    }

    #[test]
    fn create_pour() {
        let from: GlassId = 0;
        let to: GlassId = 1;

        let result = Operation::pour(from, to);

        assert_eq!(result, Pour { from, to })
    }

    #[test]
    fn display_pour() {
        let from: GlassId = 0;
        let to: GlassId = 1;

        let op = Operation::pour(from, to);

        assert_eq!("Pour(0->1)".to_owned(), format!("{}", op))
    }

    #[test]
    #[should_panic]
    fn create_pour_invalid() {
        let from: GlassId = 0;
        Operation::pour(from, from);
    }
}

