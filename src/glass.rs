use std::cmp::min;
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, Sub};

use regex::Regex;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Glass {
    pub capacity: u32,
    pub current: u32,
}

impl Glass {

    pub fn new(current: u32, capacity: u32) -> Self {
        assert!(capacity > 0, "Capacity should be > 0");
        assert!(current <= capacity, "Current should be <= capacity");
        Self { capacity, current }
    }

    pub fn new_empty(capacity: u32) -> Glass {
        Glass::new(0, capacity)
    }

    pub fn empty(&self) -> Glass {
        Glass::new_empty(self.capacity)
    }

    pub fn fill(&self) -> Glass {
        Glass::new(self.capacity, self.capacity)
    }

    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    pub fn is_full(&self) -> bool {
        self.current == self.capacity
    }

    pub fn remaining_capacity(&self) -> u32 {
        self.capacity - self.current
    }
}

impl From<&str> for Glass {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"^(?P<current>\d*)/(?P<capacity>\d*)$").unwrap();
        let caps = re.captures(s).expect("Invalid string, expected something like '2/5'");

        let current: u32 = caps.name("current").unwrap().as_str().parse().unwrap();
        let capacity: u32 = caps.name("capacity").unwrap().as_str().parse().unwrap();

        Glass::new(current, capacity)
    }
}

impl Add<u32> for Glass {
    type Output = Glass;

    fn add(self, rhs: u32) -> Self::Output {
        let current = min(self.capacity, self.current + rhs);
        Glass::new(current, self.capacity)
    }
}

impl Sub<u32> for Glass {
    type Output = Glass;

    fn sub(self, rhs: u32) -> Self::Output {
        if rhs >= self.current {
            Glass::new_empty(self.capacity)
        } else {
            Glass::new(self.current - rhs, self.capacity)
        }
    }
}

impl Display for Glass {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}/{}", self.current, self.capacity)
    }
}

impl Hash for Glass {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.current.hash(state);
        self.capacity.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::glass::Glass;

    mod create_glass {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn create_a_glass() {
            let current = 4;
            let capacity = 7;
            let glass = Glass::new(current, capacity);

            assert_eq!(glass, Glass { current, capacity });
        }

        #[test]
        #[should_panic]
        fn create_a_glass_with_invalid_capacity() {
            Glass::new(0, 0);
        }

        #[test]
        #[should_panic]
        fn create_a_glass_with_invalid_current() {
            Glass::new(11, 10);
        }
    }

    mod glass_from_string {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn create_a_glass_from_string() {
            let current = 4;
            let capacity = 7;
            let s = format!("{}/{}", 4, 7);
            let glass = Glass::from(s.as_str());

            assert_eq!(glass, Glass { current, capacity });
        }

        #[test]
        #[should_panic]
        fn create_a_glass_from_invalid_string() {
            Glass::from("3");
        }

        #[test]
        #[should_panic]
        fn create_a_glass_from_invalid_string_2() {
            Glass::from("a/b");
        }

        #[test]
        #[should_panic]
        fn create_a_glass_from_invalid_string_3() {
            Glass::from("1/2/3");
        }
    }

    mod create_empty_glass {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn create_an_empty_glass() {
            let capacity = 1;
            let glass = Glass::new_empty(capacity);

            assert_eq!(glass, Glass { current: 0, capacity });
        }

        #[test]
        #[should_panic]
        fn create_an_empty_glass_with_invalid_capacity() {
            Glass::new_empty(0);
        }
    }

    mod glass_display {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn display_a_glass() {
            let s = "4/7";
            let glass = Glass::from(s);
            assert_eq!(s.to_owned(), format!("{}", glass));
        }
    }

    mod is_empty {
        use super::*;

        #[test]
        fn glass_empty() {
            let capacity = 1;
            let glass = Glass::new_empty(capacity);

            assert!(glass.is_empty());
        }

        #[test]
        fn glass_none_empty() {
            let capacity = 1;
            let glass = Glass::new(capacity, capacity);

            assert!(!glass.is_empty());
        }
    }

    mod is_full {
        use super::*;

        #[test]
        fn glass_full() {
            let capacity = 1;
            let glass = Glass::new(capacity, capacity);

            assert!(glass.is_full());
        }

        #[test]
        fn glass_not_full() {
            let capacity = 1;
            let glass = Glass::new(capacity - 1, capacity);

            assert!(!glass.is_full());
        }
    }

    mod remaining_capacity {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn glass_remaining_capacity() {
            let current = 7;
            let capacity = 10;
            let glass = Glass::new(current, capacity);

            assert_eq!(glass.remaining_capacity(), 3);
        }

        #[test]
        fn glass_empty_remaining_capacity() {
            let capacity = 10;
            let glass = Glass::new_empty(capacity);

            assert_eq!(glass.remaining_capacity(), capacity);
        }

        #[test]
        fn glass_full_remaining_capacity() {
            let capacity = 10;
            let glass = Glass::new(capacity, capacity);

            assert_eq!(glass.remaining_capacity(), 0);
        }
    }

    mod empty {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn glass_empty() {
            let current = 3;
            let capacity = 10;
            let glass = Glass::new(current, capacity);

            let result = glass.empty();

            assert_eq!(result, Glass { current: 0, capacity });
        }
    }

    mod fill {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn glass_fill() {
            let current = 3;
            let capacity = 10;
            let glass = Glass::new(current, capacity);

            let result = glass.fill();

            assert_eq!(result, Glass { current: capacity, capacity });
        }
    }

    mod plus {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn glass_plus_quantity() {
            let current = 3;
            let capacity = 10;
            let glass = Glass::new(current, capacity);

            let result = glass + 4;

            assert_eq!(result, Glass { current: 7, capacity });
        }

        #[test]
        fn glass_plus_quantity_overflow() {
            let current = 3;
            let capacity = 10;
            let glass = Glass::new(current, capacity);

            let result = glass + 12;

            assert_eq!(result, Glass { current: capacity, capacity });
        }
    }

    mod minus {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn glass_minus_quantity() {
            let current = 7;
            let capacity = 10;
            let glass = Glass::new(current, capacity);

            let result = glass - 4;

            assert_eq!(result, Glass { current: 3, capacity });
        }

        #[test]
        fn glass_minus_quantity_overflow() {
            let current = 3;
            let capacity = 10;
            let glass = Glass::new(current, capacity);

            let result = glass - 12;

            assert_eq!(result, Glass { current: 0, capacity });
        }
    }
}
