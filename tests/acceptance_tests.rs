use rust_sudoku::sudoku::*;

// Acceptance tests with increasingly difficult Sudokus to solve
// For now, the test will fail because the game doesn't do anything, but it gives us a a gola to works towards.
// Once the easy sudoku succeeds we can add gradually more complex Sudokus
// Examples from https://sandiway.arizona.edu/sudoku/examples.html

#[test]
fn test_can_solve_easy_sudoku() {
    let initial = r#"
...26.7.1
68..7..9.
19...45..
82.1...4.
..46.29..
5...3.28
..93...74
.4..5..36
7.3.18..."#;


    let solution = r#"
435269781
682571493
197834562
826195347
374682915
951743628
519326874
248957136
763418259"#;

    let game = Game::new("easy", initial);
    assert_eq!(solution, game.to_string());
}
