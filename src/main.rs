mod sudoku;
mod games;

fn main() {
    let game = sudoku::Game::new("easiest", games::easy_sudoku());
    println!("{:#?}", game);
}
