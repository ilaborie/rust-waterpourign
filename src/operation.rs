use crate::operation::Operation::*;

pub type GlassId = u32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
    fn create_fill() {
        let glass: GlassId = 0;

        let result = Operation::fill(glass);

        assert_eq!(result, Fill { glass })
    }

    #[test]
    fn create_pour() {
        let from: GlassId = 0;
        let to: GlassId = 1;

        let result = Operation::pour(from, to);

        assert_eq!(result, Pour { from, to })
    }

    #[test]
    #[should_panic]
    fn create_pour_invalid() {
        let from: GlassId = 0;
        Operation::pour(from, from);
    }
}

