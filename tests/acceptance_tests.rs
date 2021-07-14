use rust_sudoku::sudoku::Game;
use rust_sudoku::games::*;

// Acceptance tests with increasingly difficult Sudokus to solve
// For now, the test will fail because the game doesn't do anything, but it gives us a a goal to works towards.
// Once the easy sudoku succeeds we can add gradually more complex Sudokus
// Examples from https://sandiway.arizona.edu/sudoku/examples.html

#[test]
fn test_can_solve_easy_sudoku() {
    let solution = r#"
435269781
682571493
197834562
826195347
374682915
951743628
519326874
248957136
763418259
"#;

    let game = Game::new("easy", easy_sudoku());
    assert_eq!(solution, game.to_string());
}
