mod session_1;
mod session_2;
mod session_4;
mod session_5;
use colored::*;

fn main() {
    println!("\n{}\n", "Hello, world!".bold().italic().blue());

    // session - 1
    session_1::guessing_game();

    // session - 2
    session_2::nth_fibonacci_number();
    session_2::celsius_to_farenhite();
    session_2::farenhite_to_celsius();
    session_2::christmas_carol_song();

    // session - 4
    println!("{}", session_4::first_word("world!"));

    // session - 5
    session_5::enums::main();
    session_5::structs::main();
    session_5::generics::main();
    session_5::vectors::main();
}
