use std::time::Instant;

mod games;
mod square_value;
mod sudoku;

fn main() {
    let mut game = sudoku::Game::new("easiest", games::easy_sudoku());

    let now = Instant::now();
    game.solve();
    let elapsed = now.elapsed().as_millis();
    println!("Solution for game '{}' in {} ms:{}", game.name, elapsed, game);
}
