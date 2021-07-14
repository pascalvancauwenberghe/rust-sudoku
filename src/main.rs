mod sudoku;
mod games;
mod square_value;

fn main() {
    let game = sudoku::Game::new("easiest", games::easy_sudoku());
    println!("Solution for game '{}':{}", game.name, game);
}
