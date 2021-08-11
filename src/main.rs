use rust_sudoku::games;
use rust_sudoku::sudoku::Game;
use std::time::Instant;

fn main() {
    solve("easiest 1", games::easy_sudoku(),false);
    solve("easiest 2", games::easy_sudoku2(),false);
    solve("intermediate", games::intermediate_sudoku1(),false);
    solve("difficult", games::difficult_sudoku1(),false);
    solve("not fun", games::not_fun_sudoku1(),false);
}

fn log(msg: &str) {
    println!("{}",msg) ;
}

fn solve(name: &str, initial_values: &str,logging : bool) {
    let mut game = Game::new(name, initial_values);
    if logging {
        game.logger(log);
    }
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
