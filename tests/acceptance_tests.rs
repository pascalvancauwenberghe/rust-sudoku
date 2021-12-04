use rust_sudoku::games::*;
use rust_sudoku::sudoku::Game;

// Acceptance tests with increasingly difficult Sudokus to solve

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

#[test]
fn test_can_solve_intermediate_sudoku() {
    let mut game = Game::new("intermediate", intermediate_sudoku1());
    game.solve();
    assert_eq!(intermediate_sudoku1_solution(), game.to_string());
}

#[test]
fn test_can_solve_difficult_sudoku() {
    let mut game = Game::new("difficult", difficult_sudoku1());
    game.solve();
    assert_eq!(difficult_sudoku1_solution(), game.to_string());
}

#[test]
fn test_can_solve_notfun_sudoku() {
    let mut game = Game::new("not fun", not_fun_sudoku1());
    game.solve();
    assert_eq!(not_fun_sudoku1_solution(), game.to_string());
}

#[test]
fn test_can_solve_computer_freaks_out_sudoku() {
    let mut game = Game::new("computer freaks out", computer_freaks_out());
    game.solve();
    assert_eq!(computer_freaks_out_solution(), game.to_string());
}

#[test]
fn test_can_solve_given_36_digits() {
    let mut game = Game::new("given_36_digits", given_36_digits());
    game.solve();
    assert_eq!(given_36_digits_solution(), game.to_string());
}

#[test]
fn test_can_solve_tatooine_sunset() {
    let mut game = Game::new("Tatooine sunset", tatooine_sunset());
    game.solve();
    assert_eq!(tatooine_sunset_solution(), game.to_string());
}

#[test]
fn test_can_solve_kingda_ka() {
    let mut game = Game::new("Kingda Ka", kingda_ka());
    game.solve();
    assert_eq!(kingda_ka_solution(), game.to_string());
}

#[test]
fn test_can_solve_jovial_negative() {
    let mut game = Game::new("Jovial Negative", jovial_negative());
    game.solve();
    assert_eq!(jovial_negative_solution(), game.to_string());
}
