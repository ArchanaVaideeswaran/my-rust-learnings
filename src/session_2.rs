use std::io;
use colored::*;

pub fn nth_fibonacci_number() {
  println!("\n{}\n", "nth Fibonacci Number".bold().italic().blue());
  println!("Enter a value for 'n'");

  let mut n = String::new();
  io::stdin().read_line(&mut n).expect("Failed to read value 'n'");
  let n: u32 = n.trim().parse().expect("Please enter a number");

  let mut  a : u32= 1;
  let mut b: u32 = 1;
  let mut c: u32 = a + b;

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
      for _ in 3..n {
          a = b;
          b = c;
          c = a + b;
      }
      println!(
          "{}{} {}", 
          n.to_string().bold().italic().green(),
          "th Fibonacci number is:".bold().italic().green(),
          c.to_string().bold().italic().green()
      );
  }
}

pub fn celsius_to_farenhite() {
  println!("\n{}\n", "Celsius to Farenhite".bold().italic().blue());
  println!("Enter temperature in °C");

  let mut tempc = String::new();
  io::stdin().read_line(&mut tempc).expect("Failed to read imput");
  let tempc: f64 = tempc.trim().parse().expect("Please enter a number");

  let tempf: f64 = tempc * 9.0 / 5.0 + 32.0;

  println!("{}{} = {}{}", 
    tempc.to_string().bold().italic().green(), 
    "°C".bold().italic().green(), 
    tempf.to_string().bold().italic().green(), 
    "°F".bold().italic().green()
  );
}

pub fn farenhite_to_celsius() {
  println!("\n{}\n", "Farenhite to Celsius".bold().italic().blue());
  println!("Enter temperature in °F");

  let mut tempf = String::new();
  io::stdin().read_line(&mut tempf).expect("Failed to read imput");
  let tempf: f64 = tempf.trim().parse().expect("Please enter a number");

  let tempc: f64 = (tempf - 32.0) * 5.0 / 9.0;

  println!("{}{} = {}{}", 
    tempf.to_string().bold().italic().green(), 
    "°F".bold().italic().green(), 
    tempc.to_string().bold().italic().green(), 
    "°C".bold().italic().green()
  );
}

pub fn christmas_carol_song() {
  print!("\n{}\n", "Christmas carol song".bold().italic().blue());
  let lyrics = [
        ("first", "and a Partridge in a Pear Tree"),
        ("second", "2 Turtle Doves"),
        ("third", "3 French Hens"),
        ("fourth", "4 Calling Birds"),
        ("fifth", "5 Golden Rings"),
        ("sixth", "6 Geese a Laying"),
        ("seventh", "7 Swans a Swimming"),
        ("eighth", "8 Maids a Milking"),
        ("ninth", "9 Ladies Dancing"),
        ("tenth", "10 Lords a Leaping"),
        ("eleventh", "11 Pipers Piping"),
        ("twelfth", "12 Drummers Drumming")
    ];

    for i in 0..lyrics.len() {
      print!("\n");
      println!("On the {} day of Christmas", lyrics[i].0);
      println!("my true love sent to me:");
      for j in 0..i+1 {
        if i == 0 {
          println!("A {}", &lyrics[0].1[6..]);
        } else {
            println!("{}", lyrics[i-j].1);
        }
      }
    }
}