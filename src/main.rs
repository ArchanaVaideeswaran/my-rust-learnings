use std::io;
use rand::Rng; // Rng is a trait (interface)
use std::cmp::Ordering; // Ordering is an enum with 3 values  "less", "greater", "equal"

fn main() {
    println!("Hello, world!");
    guessing_game(); // session - 1
}

fn guessing_game() {
    println!("Guessing Game!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("Secret number: {}", secret_num);
    println!("Guess the secret number within 1 - 100 range");

    loop {
        println!("Enter your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read your guess");
        let guess: u32 = guess.trim().parse().expect("Please enter a number");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed right");
                break;
            }
        }
    }
}