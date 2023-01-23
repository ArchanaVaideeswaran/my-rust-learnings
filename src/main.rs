mod rock_paper_scissors;
mod session_1;
mod session_2;
mod session_4;
mod session_5;
use std::io;

use colored::*;

fn main() {
    println!("\n{}\n", "Hello, world!".bold().italic().blue());

    println!("Please choose one of the following options:");
    println!("1) guessing_game");
    println!("2) nth_fibonacci_number");
    println!("3) celsius_to_farenhite");
    println!("4) farenhite_to_celsius");
    println!("5) christmas_carol_song");
    println!("6) first_word");
    println!("7) enums");
    println!("8) structs");
    println!("9) generics");
    println!("10) vectors");
    println!("11) traits");
    println!("12) modules");
    println!("13) nested_modules");
    println!("14) hashmaps");
    println!("15) rock_paper_scissors");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read user's choice.");

    let choice = match choice.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice!");
            return;
        }
    };

    if choice < 1 || choice > 15 {
        println!("Invalid choice!");
        return;
    }

    match choice {
        1 => session_1::guessing_game(),
        2 => session_2::nth_fibonacci_number(),
        3 => session_2::celsius_to_farenhite(),
        4 => session_2::farenhite_to_celsius(),
        5 => session_2::christmas_carol_song(),
        6 => {
            println!("Enter a set of words");
            let mut words = String::new();
            io::stdin()
                .read_line(&mut words)
                .expect("Failed to read user's choice.");
            session_4::first_word(words.as_str());
        }
        7 => session_5::enums::main(),
        8 => session_5::structs::main(),
        9 => session_5::generics::main(),
        10 => session_5::vectors::main(),
        11 => session_5::traits::main(),
        12 => session_5::modules::main(),
        13 => session_5::nested_modules::main(),
        14 => session_5::hashmaps::main(),
        15 => rock_paper_scissors::play(),
        _ => println!("Invalid input"),
    }
}
