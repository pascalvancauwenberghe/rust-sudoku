use std::time::Instant;
use rust_sudoku::sudoku::Game;
use rust_sudoku::games;

fn main() {
    solve("easiest 1", games::easy_sudoku());
    solve("easiest 2", games::easy_sudoku2());
    solve("intermediate", games::intermediate_sudoku1());
    solve("difficult", games::difficult_sudoku1());
}

fn solve(name: &str, initial_values: &'static str) {
    let mut game = Game::new(name, initial_values);
    let now = Instant::now();
    game.solve();
    let elapsed = now.elapsed().as_millis();
    if game.solved() {
        println!("Solution for game '{}' in {} ms:{}", game.name, elapsed, game);
    } else {
        println!("No solution for game '{}' in {} ms. Score = {} :{:?}", game.name, elapsed, game.possibilities(), game);
    }
}
