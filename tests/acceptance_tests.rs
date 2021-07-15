use rust_sudoku::games::*;
use rust_sudoku::sudoku::Game;

// Acceptance tests with increasingly difficult Sudokus to solve
// For now, the test will fail because the game doesn't do anything, but it gives us a a goal to works towards.
// Once the easy sudoku succeeds we can add gradually more complex Sudokus


#[test]
fn test_can_solve_easy_sudoku() {
    let mut game = Game::new("easy", easy_sudoku());
    game.solve();
    assert_eq!(easy_sudoku_solution(), game.to_string());
}

#[test]
fn test_can_solve_second_easy_sudoku() {
    let mut game = Game::new("easy2", easy_sudoku2());
    game.solve();
    assert_eq!(easy_sudoku2_solution(), game.to_string());
}