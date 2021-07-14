// A Square Value can contain a number 1..9
// Each boolean in possible_values indicates if the corresponding value can be in the cell
// We start with assuming that any of the 1..9 values is possible and reduce the possibilities with constraints

use std::ops::RangeInclusive;

#[derive(Debug, Copy, Clone)]
pub struct SquareValue {
    possible_values: [bool; 9],
}

impl SquareValue {
    pub fn new() -> Self {
        Self {
            // By default, every value is possible
            possible_values: [true; 9],
        }
    }

    // Value is 1-base, array of possible values is 0-based
    const ALL_VALUES: RangeInclusive<usize> = 1..=9;

    fn position_of_value(value: usize) -> usize {
        value - 1
    }

    // Returns true if only one of the 9 possibilities is available
    pub fn has_known_value(self: &Self) -> bool {
        self.possible_values.iter().filter(|v| **v).count() == 1
    }

    // Set the possibilities so that only the given value is possible. Sets the initial given values
    pub fn set_known_value(self: &mut Self, value: usize) {
        for v in SquareValue::ALL_VALUES {
            self.possible_values[SquareValue::position_of_value(v)] = false;
        }
        self.possible_values[SquareValue::position_of_value(value)] = true;
    }

    // Return the number contained in the value
    // If more than one value is still possible (!has_known_value), returns the lowest possible value
    pub fn value(self: &Self) -> usize {
        for v in SquareValue::ALL_VALUES {
            if self.possible_values[SquareValue::position_of_value(v)] {
                return v;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_values() {
        for v in 1..=9 {
            let mut value = SquareValue::new();

            assert!(!value.has_known_value());

            value.set_known_value(v);

            assert!(value.has_known_value());

            assert_eq!(v, value.value());
        }
    }
}