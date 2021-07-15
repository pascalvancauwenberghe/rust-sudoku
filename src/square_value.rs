// A Square Value can contain a number 1..9
// Each boolean in possible_values indicates if the corresponding value can be in the cell
// We start with assuming that any of the 1..9 values is possible and reduce the possibilities with constraints

use std::ops::RangeInclusive;
use std::fmt;

#[derive(Copy, Clone)]
pub struct SquareValue {
    possible_values: [bool; 9],
    pub row: usize,
    pub col: usize,
    propagated: bool,
}

impl SquareValue {
    pub fn new() -> Self {
        Self {
            // By default, every value is possible
            possible_values: [true; 9],
            row: 0,
            col: 0,
            propagated: false,
        }
    }

    // Value is 1-base, array of possible values is 0-based
    const ALL_VALUES: RangeInclusive<usize> = 1..=9;

    fn position_of_value(value: usize) -> usize {
        value - 1
    }

    pub fn at(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    // Returns true if only one of the 9 possibilities is available
    pub fn has_known_value(&self) -> bool {
        self.possibilities() == 1
    }

    pub fn possibilities(&self) -> usize {
        self.possible_values.iter().filter(|v| **v).count()
    }

    // Set the possibilities so that only the given value is possible. Sets the initial given values
    pub fn set_known_value(&mut self, value: usize) {
        for v in SquareValue::ALL_VALUES {
            self.possible_values[SquareValue::position_of_value(v)] = false;
        }
        self.possible_values[SquareValue::position_of_value(value)] = true;
    }

    // Return the number contained in the value
    // If more than one value is still possible (!has_known_value), returns the lowest possible value
    pub fn value(&self) -> usize {
        for v in SquareValue::ALL_VALUES {
            if self.possible_values[SquareValue::position_of_value(v)] {
                return v;
            }
        }
        0
    }

    // Remove possible value because another square in the same row/column/subgrid already has the value to maintain distinct constraint
    pub fn cant_have_value(&mut self, value: usize) {
        self.possible_values[SquareValue::position_of_value(value)] = false;
    }

    // Known values must be "propagated": the squares's value must be removed from the possibilities of squares that have a "distinct" relation with it
    // We keep track of which squares have been propagated
    pub fn needs_to_be_propagated(&self) -> bool {
        !self.propagated && self.has_known_value()
    }

    pub fn has_been_propagated(&mut self) {
        self.propagated = true;
    }

    pub fn is_possibly(&self, value: usize) -> bool {
        self.possible_values[SquareValue::position_of_value(value)]
    }
}

impl Default for SquareValue {
    fn default() -> Self {
        SquareValue::new()
    }
}

impl fmt::Debug for SquareValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let placeholder = if self.has_known_value() { '_' } else { '.' };
        for value in 1..=9 {
            if self.is_possibly(value) {
                output.push_str(&value.to_string());
            } else {
                output.push(placeholder);
            }
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_has_a_position_in_grid() {
        let mut value = SquareValue::new();
        value.at(4, 7);
        assert_eq!(4, value.row);
        assert_eq!(7, value.col);
    }

    #[test]
    fn test_create_default_values() {
        for v in 1..=9 {
            let mut value = SquareValue::new();

            assert!(!value.has_known_value());
            for possible_value in 1..=9 {
                assert!(value.is_possibly(possible_value));
            }

            value.set_known_value(v);

            assert!(value.has_known_value());

            assert_eq!(v, value.value());

            for possible_value in 1..=9 {
                assert_eq!(possible_value == v, value.is_possibly(possible_value));
            }
        }
    }

    #[test]
    fn test_propagating_known_values() {
        let mut value = SquareValue::new();

        assert_eq!(false, value.needs_to_be_propagated());

        value.set_known_value(5);
        assert_eq!(true, value.needs_to_be_propagated());

        value.has_been_propagated();
        assert_eq!(false, value.needs_to_be_propagated());
    }

    #[test]
    fn test_reducing_possibilities_due_to_constraints() {
        let mut value = SquareValue::new();

        assert!(!value.has_known_value());
        value.cant_have_value(5);
        assert!(!value.has_known_value());

        for v in 1..=9 {
            if v != 7 {
                value.cant_have_value(v);
            }
        }

        assert!(value.has_known_value());
        assert_eq!(7, value.value());
    }
}
