use std::io;
use rand::Rng; // Rng is a trait (interface)
use std::cmp::Ordering; // Ordering is an enum with 3 values  "less", "greater", "equal"
use colored::*;

pub fn main() {
  println!("Guessing Game!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("Secret number: {}", secret_num);
    println!("Guess the secret number within 1 - 100 range");

    loop {
        println!("{}", "Enter your guess".bold().blue());

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read your guess");
        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "Too small".bold().red()),
            Ordering::Greater => println!("{}", "Too big".bold().red()),
            Ordering::Equal => {
                println!("{}", "You guessed right!".bold().italic().green());
                break;
            }
        }
    }
}