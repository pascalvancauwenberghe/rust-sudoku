use std::time::Instant;
use rust_sudoku::sudoku::Game;
use rust_sudoku::games;

fn main() {
    solve(&mut Game::new("easiest 1", games::easy_sudoku()));
    solve(&mut Game::new("easiest 2", games::easy_sudoku2()));
}

fn solve(game: &mut Game) {
    let now = Instant::now();
    game.solve();
    let elapsed = now.elapsed().as_millis();
    println!("Solution for game '{}' in {} ms:{}", game.name, elapsed, game);
}
