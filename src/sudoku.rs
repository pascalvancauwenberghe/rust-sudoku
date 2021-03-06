use crate::square_value::SquareValue;
use std::fmt;
use std::ops::RangeInclusive;

// A sudoku game has a name and 9x9 squares with values
// The depth indicates how many levels of 'guesses' were needed to find solution. 0 = no guessing
// You can optionally provide a logger function to output intermediate steps
// logging flag allows for quick check if logging is enabled so that we don't pay the overhead of formatting output
pub struct Board {
    pub name: String,
    values: [SquareValue; 81],
    pub depth: usize,
    report: fn(&str),
    logging: bool,
}

pub struct Game {
    board: Board,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NakedPair {
    position1: usize,
    position2: usize,
    value1: usize,
    value2: usize,
}

pub struct Experimental {
    pub value: usize,
    pub branching: usize,
    pub trying: SquareValue,
    pub board: Board,
}

impl Experimental {
    pub fn new(board: Board, trying: SquareValue, value: usize, branching: usize) -> Self {
        Self { value, branching, trying, board }
    }

    pub fn order(&self) -> usize {
        self.board.depth * 10000 + self.branching * 1000 + (self.trying.row * self.trying.col) + self.value
    }
}

impl Game {
    pub fn new(game_name: &str, initial: &str) -> Self {
        Self {
            board: Board::new(game_name, initial),
        }
    }

    // Provide a logger function for intermediate steps
    pub fn logger(&mut self, output: fn(&str)) {
        self.board.logger(output);
    }

    pub fn solve(&mut self) -> bool {
        let solved = self.board.solve();
        if !solved {
            // Last resort: guess a value and recurse
            // heuristic: guess squares with the least number of possibilities, so as to maximize odds of guessing right
            // Once we make a guess, we first apply out solving techniques again.
            // Search is performed breadth-first:
            // - If we can't solve the initial problem (depth == 0), generate "experiments" (original problem + 1 guessed value) at level 1
            // - Each time we get stuck at depth N, generate experiments at depth N+1
            // - An experiment that results in a solved board, copies the experimental board in the result and returns
            // - An experiment that results in inconsistencies is dropped
            // - The list of experiments is kept sorted so that the experiments with smallest depth are at the back, to be popped and tried
            let mut experiments = self.board.generate_experiments();
            experiments.sort_by(|a, b| b.order().partial_cmp(&a.order()).unwrap());
            while !self.board.solved() && !experiments.is_empty() {
                if let Some(mut experimental) = experiments.pop() {
                    if experimental.board.solve() {
                        self.board.assign(&experimental.board);
                        return true;
                    } else if !experimental.board.contains_contradiction() {
                        experiments.append(&mut experimental.board.generate_experiments());
                        experiments.sort_by(|a, b| b.order().partial_cmp(&a.order()).unwrap());
                    }
                }
            }
        }
        self.board.solved()
    }

    pub fn solved(&self) -> bool {
        self.board.solved()
    }

    pub fn name(&self) -> String {
        self.board.name.clone()
    }

    pub fn possibilities(&self) -> usize {
        self.board.possibilities()
    }

    pub fn depth(&self) -> usize {
        self.board.depth
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Self {
            name: self.name.clone(),
            values: self.values,
            depth: self.depth + 1,
            report: self.report,
            logging: self.logging,
        }
    }
}

// checks for self.logging are separate for readability
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::collapsible_if)]
impl Board {
    const ALL_VALUES: RangeInclusive<usize> = 1..=9;
    const ALL_ROWS: RangeInclusive<usize> = 1..=9;
    const ALL_COLUMNS: RangeInclusive<usize> = 1..=9;

    // Default empty logger implementation
    fn silent(_str: &str) {}

    pub fn new(game_name: &str, initial: &str) -> Self {
        let mut result = Self {
            name: game_name.to_string(),
            values: [SquareValue::new(); 81],
            depth: 0,
            report: Board::silent,
            logging: false,
        };
        for row in Board::ALL_ROWS {
            for col in Board::ALL_COLUMNS {
                result.values[Board::position_of(row, col)].at(row, col);
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

    // Provide a logger function for intermediate steps
    pub fn logger(&mut self, output: fn(&str)) {
        self.report = output;
        self.logging = true;
    }

    fn report(&self, str: String) {
        let f = self.report;
        f(&str);
    }

    fn assign(&mut self, other: &Board) {
        self.values = other.values;
        self.depth = other.depth;
    }

    fn position_of(row: usize, col: usize) -> usize {
        (row - 1) * 9 + (col - 1)
    }

    pub fn row_of(pos: usize) -> usize { pos / 9 + 1 }
    pub fn col_of(pos: usize) -> usize { pos % 9 + 1 }
    pub fn rowgrid_of(pos: usize) -> usize { (pos / 9) / 3 }
    pub fn colgrid_of(pos: usize) -> usize { (pos % 9) / 3 }

    pub fn possibilities(&self) -> usize {
        self.values.iter().map(|c| c.possibilities()).sum()
    }

    pub fn solved(&self) -> bool {
        self.values.iter().all(|c| c.has_known_value())
    }

    pub fn contains_contradiction(&self) -> bool {
        self.values.iter().any(|c| c.is_contradiction())
    }

    fn all_values_in_row(row: usize) -> [usize; 9] {
        let mut result = [0; 9];

        for col in Board::ALL_ROWS {
            result[col - 1] = Board::position_of(row, col);
        }

        result
    }

    fn all_values_in_column(col: usize) -> [usize; 9] {
        let mut result = [0; 9];

        for row in Board::ALL_ROWS {
            result[row - 1] = Board::position_of(row, col);
        }
        result
    }

    fn all_values_in_subgrid(rowgrid: usize, colgrid: usize) -> [usize; 9] {
        let mut result = [0; 9];

        let row_offset = rowgrid * 3;
        let col_offset = colgrid * 3;

        let mut pos = 0;
        for r in 1..=3 {
            for c in 1..=3 {
                result[pos] = Board::position_of(row_offset + r, col_offset + c);
                pos += 1;
            }
        }

        result
    }

    // Sudoku solver returns
    // true -> solution found
    // false -> no solution found
    pub fn solve(&mut self) -> bool {
        if self.logging {
            self.report(format!("Solving board at depth {} {:?} ", self.depth, self));
        }
        // As long as we're making progress, apply our solving techniques
        let mut progress_made = true;
        let mut pairs: Vec<NakedPair> = Vec::new();
        while progress_made {
            // Technique 1: propagate unit values to reduce possibilities in same row, column and subgrid
            progress_made = self.propagate_all_known_values();
            if progress_made {
                if self.logging {
                    self.report(format!("After propagating known values {:?}", self));
                }
                if self.solved() {
                    return true;
                }
                if self.contains_contradiction() {
                    if self.logging {
                        self.report(format!("!! Board results in contradiction. Backtracking from level {} !!", self.depth));
                    }
                    return false;
                }

                progress_made = self.find_naked_pairs(&mut pairs);
                if progress_made {
                    if self.logging {
                        self.report(format!("After cleaning up naked pairs {:?}", self));
                    }
                }

                // Technique 2: possibilities may have been reduced so that 'singletons' can be found
                // When a singleton is promoted to value, this value must be propagated
                progress_made |= self.promote_singletons();
                if progress_made {
                    if self.logging {
                        self.report(format!("After promoting singletons {:?}", self));
                    }
                }
            }
        }
        false
    }

    fn generate_experiments(&self) -> Vec<Experimental> {
        let mut experiments: Vec<Experimental> = Vec::new();
        let candidate = self.find_cell_to_guess();
        if let Some(square) = candidate {
            let guess_position = Board::position_of(square.row, square.col);
            let branching = square.possibilities();
            for v in Board::ALL_VALUES {
                if square.can_have_value(v) {
                    if self.logging {
                        self.report(format!(
                            ">>> Guess that square ({},{}) has value {} at level {}",
                            square.row, square.col, v, self.depth
                        ));
                    }
                    let mut experimental = self.clone();
                    experimental.values[guess_position].set_known_value(v);
                    experiments.push(Experimental::new(experimental, square, v, branching));
                }
            }
        }
        experiments
    }

    fn find_cell_to_guess(&self) -> Option<SquareValue> {
        self.values
            .iter()
            .filter(|c| !c.has_known_value())
            .min_by_key(|c| c.possibilities())
            .cloned()
    }

    // The first technique is to start with known values and remove those values from the possible
    //  values in the squares in the same row, column and subgrid, to maintain the "distinct" constraint
    // Initially we know the given values. Removing possibilities may lead us to discover a new value,
    // which can then be used to remove more values. Etc.
    // So we keep propagating until we run out of values to propagate
    // This is enough to solve really simple Sudokus
    // Example: [ 1 2 3 4 ] [ 4 ] [ 1 2 3 4 ] => [ 1 2 3 ] [ 4 ] [ 1 2 3 ]

    fn propagate_all_known_values(&mut self) -> bool {
        let mut progress_made = false;
        while self.propagate_known_values() {
            progress_made = true;
        }
        progress_made
    }

    // Unit value propagation
    fn propagate_known_values(&mut self) -> bool {
        let square = self.find_cell_to_propagate();
        if let Some(value) = square {
            if self.logging {
                self.report(format!(
                    "Propagating value {} of ({},{})",
                    value.value(),
                    value.row,
                    value.col
                ));
            }
            self.propagate_known_values_in_all_except(
                Board::all_values_in_column(value.col),
                &value,
            );
            self.propagate_known_values_in_all_except(Board::all_values_in_row(value.row), &value);
            self.propagate_known_values_in_all_except(
                Board::all_values_in_subgrid(value.row_grid(), value.col_grid()),
                &value,
            );

            self.values[Board::position_of(value.row, value.col)].has_been_propagated();
            return true;
        }
        false
    }

    fn propagate_known_values_in_all_except(
        &mut self,
        positions: [usize; 9],
        except_square: &SquareValue,
    ) {
        let known_position = Board::position_of(except_square.row, except_square.col);
        let known_value = except_square.value();
        for pos in positions.iter() {
            if *pos != known_position {
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

    // The second technique spots "singletons" in row/column/subgrid
    // A "singleton" is a square that contains multiple possibilities but is the only one to have a certain value as possibility
    // We can conclude from this that the square must have that "singleton" value, as all values must be used
    // Example: [ 1 2 ] [ 1 2 3 ] [ 1 2 ] => [ 1 2 ] [ 3 ] [ 1 2 ]

    fn promote_singletons(&mut self) -> bool {
        let mut promoted = false;

        for row in Board::ALL_ROWS {
            promoted |= self.promote_singleton_in(Board::all_values_in_row(row));
        }

        for col in Board::ALL_COLUMNS {
            promoted |= self.promote_singleton_in(Board::all_values_in_column(col));
        }

        for rowgrid in 0..=2 {
            for colgrid in 0..=2 {
                promoted |=
                    self.promote_singleton_in(Board::all_values_in_subgrid(rowgrid, colgrid));
            }
        }

        promoted
    }

    fn promote_singleton_in(&mut self, positions: [usize; 9]) -> bool {
        let mut promoted = false;
        for value in Board::ALL_VALUES {
            let mut occurences = 0;
            let mut foundpos = 0;
            for pos in positions.iter() {
                if self.values[*pos].can_have_value(value) {
                    occurences += 1;
                    foundpos = *pos;
                }
            }
            if occurences == 1 && !self.values[foundpos].has_known_value() {
                self.values[foundpos].set_known_value(value);
                promoted = true;
            }
        }

        promoted
    }

    fn find_naked_pairs(&mut self, pairs: &mut Vec<NakedPair>) -> bool {
        let mut found = false;

        let before = pairs.len();
        for row in Board::ALL_ROWS {
            found |= self.find_naked_pairs_in(Board::all_values_in_row(row), pairs);
        }

        for col in Board::ALL_COLUMNS {
            found |= self.find_naked_pairs_in(Board::all_values_in_column(col), pairs);
        }

        for rowgrid in 0..=2 {
            for colgrid in 0..=2 {
                found |=
                    self.find_naked_pairs_in(Board::all_values_in_subgrid(rowgrid, colgrid), pairs);
            }
        }

        let after = pairs.len();

        if found {
            for i in before..after {
                let pair = pairs[i];
                self.report(format!(
                    "Cleaning naked pair {} and {} at ({},{}) and ({},{}) pos: {} {}",
                    pair.value1, pair.value2,
                    Board::row_of(pair.position1), Board::col_of(pair.position1),
                    Board::row_of(pair.position2), Board::col_of(pair.position2),
                    self.values[pair.position1].possibilities(),
                    self.values[pair.position2].possibilities()
                ));
                self.clean_naked_pair(pair);
                self.clean_neighbours_of_naked_pair(pair);
            }
        }

        return found;
    }

    fn find_naked_pairs_in(&mut self, positions: [usize; 9], pairs: &mut Vec<NakedPair>) -> bool {
        let mut found = false;
        for first_value in 1..9 {
            for second_value in first_value + 1..=9 {
                let mut matches: Vec<usize> = Vec::new();
                let mut single = false;
                let mut pure = true ;
                for pos in positions.iter() {
                    if self.values[*pos].can_have_value(first_value) && self.values[*pos].can_have_value(second_value) {
                        matches.push(*pos);
                        pure &= self.values[*pos].possibilities() == 2 ;
                    } else if self.values[*pos].can_have_value(first_value) || self.values[*pos].can_have_value(second_value) {
                        single = true;
                    }
                }
                if matches.len() == 2 && (!single || pure) {
                    let pair = NakedPair {
                        position1: matches[0],
                        position2: matches[1],
                        value1: first_value,
                        value2: second_value,
                    };
                    if !pairs.contains(&pair) {
                        pairs.push(pair);
                        found = true;
                    }
                }
            }
        }

        found
    }
    fn clean_naked_pair(&mut self, pair: NakedPair) {
        for v in Board::ALL_VALUES {
            if v != pair.value1 && v != pair.value2 {
                self.values[pair.position1].cant_have_value(v);
                self.values[pair.position2].cant_have_value(v);
            }
        }
    }

    fn clean_neighbours_of_naked_pair(&mut self, pair: NakedPair) {
        if Board::row_of(pair.position1) == Board::row_of(pair.position2) {
            self.clean_neighbours(pair, Board::all_values_in_row(Board::row_of(pair.position1)));
        }
        if Board::col_of(pair.position1) == Board::col_of(pair.position2) {
            self.clean_neighbours(pair, Board::all_values_in_column(Board::col_of(pair.position1)));
        }
        if Board::colgrid_of(pair.position1) == Board::colgrid_of(pair.position2) &&
            Board::rowgrid_of(pair.position1) == Board::rowgrid_of(pair.position2) {
            self.clean_neighbours(pair, Board::all_values_in_subgrid(Board::rowgrid_of(pair.position1), Board::colgrid_of(pair.position1)));
        }
    }

    fn clean_neighbours(&mut self, pair: NakedPair, positions: [usize; 9]) {
        for pos in positions.iter() {
            if *pos != pair.position1 && *pos != pair.position2 {
                self.values[*pos].cant_have_value(pair.value1);
                self.values[*pos].cant_have_value(pair.value2);
            }
        }
    }
}

// Default toString implementation. Prints out known values as digit and unknown values as '.'. One row per line, same as the input format
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output += "\n";
        for row in Board::ALL_ROWS {
            for col in Board::ALL_COLUMNS {
                let value = &self.values[Board::position_of(row, col)];
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

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.board.fmt(f)
    }
}

// Debug output shows the grid with subdivisions + all possible values of each cell
// If a square is known, the other possibilities are shown as '_', otherwise as '.'.
// E.g.
// ..3..67.. = 3 possibilities left
// _____6___ = value is known to be 6
// XXXXXXXXX = inconsistent state, nothing is possible
// An inconsistent square (without possibilities) is shown as 'XXXXXXXXX'
impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("\n  ------------------------------------------- Level {} --------------------------------------------\n", self.depth));
        for row in Board::ALL_ROWS {
            output.push_str(" | ");
            for col in Board::ALL_COLUMNS {
                let square = self.values[Board::position_of(row, col)];
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

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.board.fmt(f)
    }
}

// Parses a multi-line string with the starting values of the Sudoku
// Digit => value of the digit 1..9
// '.'   => 0
// values are arranged row per row
// Empty (less than 9 chars) are skipped
// The input doesn't have to contain 9 rows. If rows are missing, they are assumed to be empty (= 0 values)
fn parse_initial_sudoku_values(values: &str) -> [usize; 81] {
    let lines = values.lines();
    let mut result: [usize; 81] = [0; 81];
    let mut row = 0;
    for line in lines {
        if line.len() >= 9 {
            row += 1;
            for col in Board::ALL_COLUMNS {
                let kar = line.chars().nth(col - 1).unwrap();
                if ('1'..='9').contains(&kar) {
                    result[Board::position_of(row, col)] = kar.to_digit(10).unwrap() as usize;
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
        let game = Board::new("easy", easy_sudoku());
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
        let game = Board::new("easy", easy_sudoku());
        assert_eq!(easy_sudoku(), game.to_string());
    }

    #[test]
    fn test_game_propagates_known_values() {
        let mut game = Board::new("easy", easy_sudoku());
        game.solve();
        for col in 1..9 {
            let cell = game.values[Board::position_of(1, col)];
            if !cell.has_known_value() {
                assert_eq!(9 - 4, cell.possibilities()); // 4 cells are known
            }
        }
        assert!(game.solved());
        assert_eq!(81, game.possibilities());
        assert_eq!(easy_sudoku_solution(), game.to_string());
    }
}
