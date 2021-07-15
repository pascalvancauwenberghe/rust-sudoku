use crate::square_value::SquareValue;
use std::fmt;

const DEBUG: bool = false;

pub struct Game {
    pub name: String,
    values: [SquareValue; 81],
}

impl Clone for Game {
    fn clone(&self) -> Game {
        Self {
            name: self.name.clone(),
            values: self.values,
        }
    }
}

impl Game {
    pub fn new(game_name: &str, initial: &'static str) -> Self {
        let mut result = Self {
            name: game_name.to_string(),
            values: [SquareValue::new(); 81],
        };
        for row in 1..=9 {
            for col in 1..=9 {
                result.values[Game::position_of(row, col)].at(row, col);
            }
        }
        let parsed = parse_initial_sudoku_values(initial);
        for (pos, parsed_value) in parsed.iter().enumerate() {
            if *parsed_value != 0 {
                result.values[pos].set_known_value(*parsed_value);
            }
        }
        result
    }

    fn assign(&mut self, other: &Game) {
        self.values = other.values;
    }

    fn position_of(row: usize, col: usize) -> usize {
        (row - 1) * 9 + (col - 1)
    }

    pub fn possibilities(&self) -> usize {
        self.values.iter().map(|c| c.possibilities()).sum()
    }

    pub fn solved(&self) -> bool {
        !self.values.iter().any(|c| !c.has_known_value())
    }

    pub fn contains_contradiction(&self) -> bool {
        self.values.iter().any(|c| c.is_contradiction())
    }

    fn all_values_in_row(row: usize) -> [usize; 9] {
        let mut result = [0; 9];

        result[0] = Game::position_of(row, 1);
        result[1] = Game::position_of(row, 2);
        result[2] = Game::position_of(row, 3);
        result[3] = Game::position_of(row, 4);
        result[4] = Game::position_of(row, 5);
        result[5] = Game::position_of(row, 6);
        result[6] = Game::position_of(row, 7);
        result[7] = Game::position_of(row, 8);
        result[8] = Game::position_of(row, 9);

        result
    }

    fn all_values_in_column(col: usize) -> [usize; 9] {
        let mut result = [0; 9];

        result[0] = Game::position_of(1, col);
        result[1] = Game::position_of(2, col);
        result[2] = Game::position_of(3, col);
        result[3] = Game::position_of(4, col);
        result[4] = Game::position_of(5, col);
        result[5] = Game::position_of(6, col);
        result[6] = Game::position_of(7, col);
        result[7] = Game::position_of(8, col);
        result[8] = Game::position_of(9, col);

        result
    }

    fn all_values_in_subgrid(rowgrid: usize, colgrid: usize) -> [usize; 9] {
        let mut result = [0; 9];

        result[0] = Game::position_of(rowgrid * 3 + 1, colgrid * 3 + 1);
        result[1] = Game::position_of(rowgrid * 3 + 1, colgrid * 3 + 2);
        result[2] = Game::position_of(rowgrid * 3 + 1, colgrid * 3 + 3);
        result[3] = Game::position_of(rowgrid * 3 + 2, colgrid * 3 + 1);
        result[4] = Game::position_of(rowgrid * 3 + 2, colgrid * 3 + 2);
        result[5] = Game::position_of(rowgrid * 3 + 2, colgrid * 3 + 3);
        result[6] = Game::position_of(rowgrid * 3 + 3, colgrid * 3 + 1);
        result[7] = Game::position_of(rowgrid * 3 + 3, colgrid * 3 + 2);
        result[8] = Game::position_of(rowgrid * 3 + 3, colgrid * 3 + 3);

        result
    }

    // The first technique is to start with known values and remove those values from the possible
    //  values in the squares in the same row, column and subgrid, to maintain the "distinct" constraint
    // Initially we know the given values. Removing possibilities may lead us to discover a new value,
    // which can then be used to remove more values. Etc.
    // So we keep propagating until we run of values to propagate
    // This is enough to solve really simple Sudokus
    // Example: [ 1 2 3 4 ] [ 4 ] [ 1 2 3 4 ] => [ 1 2 3 ] [ 4 ] [ 1 2 3 ]

    // The second technique spots "singletons" in row/column/subgrid
    // A "singleton" is a square that contains multiple possibilities but is the only one to have a certain value as possibility
    // We can conclude from this that the square must have that "singleton" value, as all values must be used
    // Example: [ 1 2 ] [ 1 2 3] [ 1 2 ] => [ 1 2 ] [ 3 ] [ 1 2 ]
    pub fn solve(&mut self) -> &Self {
        if DEBUG {
            println!("Solving {:?}", self);
        }
        let mut progress_made = true;
        while progress_made {
            progress_made = self.propagate();
            if progress_made {
                if DEBUG {
                    println!("After propagating known values {:?}", self);
                }
                if self.contains_contradiction() || self.solved() {
                    return self;
                }
                progress_made = self.promote_singletons();
                if progress_made {
                    if DEBUG {
                        println!("After promoting singeletons {:?}", self);
                    }
                    if self.contains_contradiction() || self.solved() {
                        return self;
                    }
                }
            }
        }

        // Last resort: guess a value and recurse
        // heuristic: guess squares with the least number of possibilities, so as to maximize odds of guessing right
        let candidate = self.find_cell_to_guess();
        if let Some(square) = candidate {
            for v in 1..=9 {
                if square.can_have_value(v) {
                    if DEBUG {
                        println!(
                            ">>> Guessing that square ({},{}) has value {}",
                            square.row, square.col, v
                        );
                    }
                    let mut experimental = self.clone();
                    experimental.values[Game::position_of(square.row, square.col)]
                        .set_known_value(v);
                    experimental.solve();
                    if experimental.solved() && !self.contains_contradiction() {
                        self.assign(&experimental);
                        return self;
                    } else {
                        if DEBUG {
                            println!("<<< Backtracking");
                        }
                    }
                }
            }
        }
        self
    }

    fn propagate(&mut self) -> bool {
        let mut progress_made = false;
        while self.propagate_known_values() {
            progress_made = true
        }
        progress_made
    }

    // Singleton promotion

    fn promote_singletons(&mut self) -> bool {
        let mut promoted = false;

        for row in 1..=9 {
            promoted |= self.promote_singleton_in(Game::all_values_in_row(row));
        }

        for col in 1..=9 {
            promoted |= self.promote_singleton_in(Game::all_values_in_column(col));
        }

        for rowgrid in 0..=2 {
            for colgrid in 0..=2 {
                promoted |=
                    self.promote_singleton_in(Game::all_values_in_subgrid(rowgrid, colgrid));
            }
        }

        promoted
    }

    fn promote_singleton_in(&mut self, positions: [usize; 9]) -> bool {
        let mut promoted = false;
        for value in 1..=9 {
            let mut occurences = 0;
            for pos in positions.iter() {
                if self.values[*pos].can_have_value(value) {
                    occurences += 1;
                }
            }
            if occurences == 1 {
                for pos in positions.iter() {
                    if self.values[*pos].can_have_value(value)
                        && !self.values[*pos].has_known_value()
                    {
                        self.values[*pos].set_known_value(value);
                        promoted = true;
                    }
                }
            }
        }

        promoted
    }

    // Unit value propagation
    fn propagate_known_values(&mut self) -> bool {
        let square = self.find_cell_to_propagate();
        if let Some(value) = square {
            if DEBUG {
                println!(
                    "Propagating value {} of ({},{})",
                    value.value(),
                    value.row,
                    value.col
                );
            }
            self.propagate_known_values_in_all_except(
                &value,
                Game::all_values_in_column(value.col),
            );
            self.propagate_known_values_in_all_except(&value, Game::all_values_in_row(value.row));
            self.propagate_known_values_in_all_except(
                &value,
                Game::all_values_in_subgrid(value.row_grid(), value.col_grid()),
            );

            self.values[Game::position_of(value.row, value.col)].has_been_propagated();
            return true;
        }
        false
    }

    fn propagate_known_values_in_all_except(
        &mut self,
        square: &SquareValue,
        positions: [usize; 9],
    ) {
        let known_value = square.value();
        for pos in positions.iter() {
            if *pos != Game::position_of(square.row, square.col) {
                self.values[*pos].cant_have_value(known_value);
            }
        }
    }

    fn find_cell_to_propagate(&self) -> Option<SquareValue> {
        self.values
            .iter()
            .find(|v| v.needs_to_be_propagated())
            .cloned()
    }

    fn find_cell_to_guess(&self) -> Option<SquareValue> {
        self.values
            .iter()
            .filter(|c| !c.has_known_value())
            .min_by_key(|c| c.possibilities())
            .cloned()
    }
}

// Default toString implementation. Does nothing now, will print current value of game, solution if found one
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output += "\n";
        for row in 1..=9 {
            for col in 1..=9 {
                let value = &self.values[Game::position_of(row, col)];
                if !value.has_known_value() {
                    output.push('.');
                } else {
                    output.push_str(&value.value().to_string());
                }
            }
            output += "\n";
        }
        write!(f, "{}", output)
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str("\n  -------------------------------------------------------------------------------------------------\n");
        for row in 1..=9 {
            output.push_str(" | ");
            for col in 1..=9 {
                let square = self.values[Game::position_of(row, col)];
                output.push_str(&format!("{:?} ", square));
                if col % 3 == 0 {
                    output.push_str(" | ");
                }
            }
            output.push('\n');
            if row % 3 == 0 {
                output.push_str(" -------------------------------------------------------------------------------------------------\n");
            }
        }

        write!(f, "{}", output)
    }
}

// Parses a multi-line string with the starting values of the Sudoku
// Digit => value of the digit 1..9
// '.'   => 0
// values are arranged row per row
fn parse_initial_sudoku_values(values: &str) -> [usize; 81] {
    let lines = values.lines();
    let mut result: [usize; 81] = [0; 81];
    let mut row = 0;
    for line in lines {
        if line.len() >= 9 {
            row += 1;
            for col in 1..=9 {
                let kar = line.chars().nth(col - 1).unwrap();
                if ('1'..='9').contains(&kar) {
                    result[Game::position_of(row, col)] = kar.to_digit(10).unwrap() as usize;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::games::*;

    #[test]
    fn test_create_game_with_name() {
        let game = Game::new("easy", easy_sudoku());
        assert_eq!("easy", game.name);
    }

    #[test]
    fn test_parse_initial_values() {
        let output = parse_initial_sudoku_values(easy_sudoku());
        assert_eq!(81, output.len());

        // First line
        assert_eq!(0, output[0 + 0]);
        assert_eq!(0, output[0 + 1]);
        assert_eq!(0, output[0 + 2]);

        assert_eq!(2, output[0 + 3]);
        assert_eq!(6, output[0 + 4]);
        assert_eq!(0, output[0 + 5]);

        assert_eq!(7, output[0 + 6]);
        assert_eq!(0, output[0 + 7]);
        assert_eq!(1, output[0 + 8]);

        // Second line
        assert_eq!(6, output[9 + 0]);
        assert_eq!(8, output[9 + 1]);
        assert_eq!(0, output[9 + 2]);

        assert_eq!(0, output[9 + 3]);
        assert_eq!(7, output[9 + 4]);
        assert_eq!(0, output[9 + 5]);

        assert_eq!(0, output[9 + 6]);
        assert_eq!(9, output[9 + 7]);
        assert_eq!(0, output[9 + 8]);

        // Middle line
        assert_eq!(0, output[36 + 0]);
        assert_eq!(0, output[36 + 1]);
        assert_eq!(4, output[36 + 2]);

        assert_eq!(6, output[36 + 3]);
        assert_eq!(0, output[36 + 4]);
        assert_eq!(2, output[36 + 5]);

        assert_eq!(9, output[36 + 6]);
        assert_eq!(0, output[36 + 7]);
        assert_eq!(0, output[36 + 8]);

        // Last line
        assert_eq!(7, output[72 + 0]);
        assert_eq!(0, output[72 + 1]);
        assert_eq!(3, output[72 + 2]);

        assert_eq!(0, output[72 + 3]);
        assert_eq!(1, output[72 + 4]);
        assert_eq!(8, output[72 + 5]);

        assert_eq!(0, output[72 + 6]);
        assert_eq!(0, output[72 + 7]);
        assert_eq!(0, output[72 + 8]);
    }

    #[test]
    fn test_parse_empty_initial_values() {
        let output = parse_initial_sudoku_values("");
        assert_eq!(81, output.len());
        for pos in 0..81 {
            assert_eq!(0, output[pos]);
        }
    }

    #[test]
    fn test_game_prints_initial_values() {
        let game = Game::new("easy", easy_sudoku());
        assert_eq!(easy_sudoku(), game.to_string());
    }

    #[test]
    fn test_game_propagates_known_values() {
        let mut game = Game::new("easy", easy_sudoku());
        game.solve();
        for col in 1..9 {
            let cell = game.values[Game::position_of(1, col)];
            if !cell.has_known_value() {
                assert_eq!(9 - 4, cell.possibilities()); // 4 cells are known
            }
        }
        assert!(game.solved());
        assert_eq!(81, game.possibilities());
        assert_eq!(easy_sudoku_solution(), game.to_string());
    }
}
