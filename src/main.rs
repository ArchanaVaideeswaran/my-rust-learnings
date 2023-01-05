mod nth_fibonacci_number;
mod guessing_game;
mod temprature_converter;
mod christmas_carol;
use colored::*;

fn main() {
    println!("\n{}\n", "Hello, world!".bold().italic().blue());

    // session - 1
    guessing_game::main();

    // session - 2
    nth_fibonacci_number::main();
    temprature_converter::celsius_to_farenhite();
    temprature_converter::farenhite_to_celsius();
    christmas_carol::main();
}