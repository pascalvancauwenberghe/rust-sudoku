use rust_sudoku::games;
use rust_sudoku::sudoku::Game;
use std::time::Instant;

fn main() {
    /*    solve("easiest 1", games::easy_sudoku());
    solve("easiest 2", games::easy_sudoku2());
    solve("intermediate", games::intermediate_sudoku1());
    solve("difficult", games::difficult_sudoku1());*/
    solve("not fun", games::not_fun_sudoku1());
}

fn solve(name: &str, initial_values: &'static str) {
    let mut game = Game::new(name, initial_values);
    let now = Instant::now();
    game.solve();
    let elapsed = now.elapsed().as_micros();
    if game.solved() {
        println!(
            "Solution for game '{}' in {} µs:{}",
            game.name, elapsed, game
        );
    } else {
        println!(
            "No solution for game '{}' in {} µs. Score = {} :{:?}",
            game.name,
            elapsed,
            game.possibilities(),
            game
        );
    }
}
