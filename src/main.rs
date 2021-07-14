mod sudoku;

fn main() {
    let game = sudoku::Game::new("easiest", "");
    println!("{:#?}", game);
}
