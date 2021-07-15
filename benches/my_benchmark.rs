use criterion::{criterion_group, criterion_main, Criterion};
use rust_sudoku::games::{difficult_sudoku1, not_fun_sudoku1};
use rust_sudoku::sudoku::Game;

fn solve_notfun_sudoku() -> Game {
    let mut game = Game::new("not fun", not_fun_sudoku1());
    game.solve();
    game
}

fn solve_difficult_sudoku() {
    let mut game = Game::new("difficult", difficult_sudoku1());
    game.solve();
}

fn notfun_benchmark(c: &mut Criterion) {
    c.bench_function("not fun", |b| b.iter(|| solve_notfun_sudoku()));
}

fn difficult_benchmark(c: &mut Criterion) {
    c.bench_function("difficult", |b| b.iter(|| solve_difficult_sudoku()));
}

criterion_group!(benches, notfun_benchmark, difficult_benchmark);
criterion_main!(benches);
