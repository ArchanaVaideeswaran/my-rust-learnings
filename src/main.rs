pub mod nth_fibonacci_number;
pub mod guessing_game;

fn main() {
    println!("Hello, world!");

    // session - 1
    guessing_game::main();

    // session - 2
    nth_fibonacci_number::main();
}