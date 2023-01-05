use std::io;
use colored::*;

pub fn main() {
    println!("\n{}\n", "nth Fibonacci Number".bold().italic().blue());
    println!("Enter a value for 'n'");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read value 'n'");
    let n: u32 = n.trim().parse().expect("Please enter a number");

    let mut  a = 1;
    let mut b = 1;
    let mut c = a + b;

    if n == 1 {
        println!(
            "{}{} {}", 
            n.to_string().bold().italic().green(),
            "th Fibonacci number is:".bold().italic().green(),
            a.to_string().bold().italic().green()
        );
    }
    else if n == 2 {
        println!(
            "{}{} {}", 
            n.to_string().bold().italic().green(),
            "th Fibonacci number is:".bold().italic().green(), 
            b.to_string().bold().italic().green()
        );
    } 
    else if n == 3 {
        println!(
            "{}{} {}", 
            n.to_string().bold().italic().green(),
            "th Fibonacci number is:".bold().italic().green(), 
            c.to_string().bold().italic().green()
        );
    } 
    else {
        let mut count = 3;
        while count < n {  
            a = b;
            b = c;
            c = a + b; 
            count += 1;          
        }
        println!(
            "{}{} {}", 
            n.to_string().bold().italic().green(),
            "th Fibonacci number is:".bold().italic().green(),
            c.to_string().bold().italic().green()
        );
    }
}