use std::io;

pub fn play() {
    println!("Welcome to the Rock-Paper-Scissors game!");

    loop {
        loop {
            println!("Please choose one of the following options:");
            println!("1) Rock");
            println!("2) Paper");
            println!("3) Scissors");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read user's choice.");

            let choice = match choice.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid choice. Please enter a number between 1 and 3.");
                    continue;
                }
            };

            if choice < 1 || choice > 3 {
                println!("Invalid choice. Please enter a number between 1 and 3.");
                continue;
            }

            let computer_choice = (rand::random::<i32>() % 3) + 1;

            println!("You chose: {}", display_choice(choice));
            println!("Computer chose: {}", display_choice(computer_choice));

            match (choice, computer_choice) {
                (1, 2) => println!("Paper beats rock. Computer wins."),
                (1, 3) => {
                    println!("Rock beats scissors. You win!");
                    break;
                }
                (2, 1) => {
                    println!("Paper beats rock. You win!");
                    break;
                }
                (2, 3) => println!("Scissors beats paper. Computer wins."),
                (3, 1) => println!("Rock beats scissors. Computer wins."),
                (3, 2) => {
                    println!("Scissors beats paper. You win!");
                    break;
                }
                (_, _) => println!("It's a tie!"),
            }
        }
        println!("Do you want to play again? (y/n)");

        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read user's choice.");

        if play_again.trim() != "y" {
            break;
        }
    }
}

fn display_choice(choice: i32) -> &'static str {
    match choice {
        1 => "Rock",
        2 => "Paper",
        3 => "Scissors",
        _ => "Invalid choice",
    }
}
