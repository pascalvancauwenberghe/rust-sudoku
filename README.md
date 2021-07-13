# rust-sudoku

This is a small Sudoku solver that uses simple constraint satisfaction techniques. The goal of the project is to learn basic (no macros, no concurrency, console application without user interaction) Rust programming techniques.

The code should

- be simple to understand, with limited Rust experience
- conform to Rust programming conventions
- solve hard Sudokus in less than 1-2 seconds on a machine that is powerful enough to serve as a development environment

Non-goals

- implement advanced Sudoku solving heuristics. See Youtube channel "[Cracking the Cryptic](https://www.youtube.com/c/CrackingTheCryptic)" if you want to know more about those
- maximally optimize the application
- develop a generic SAT/SMT application. See [Z3](https://github.com/Z3Prover/z3) and [Rust bindings for Z3](https://github.com/prove-rs/z3.rs)
