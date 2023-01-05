use std::io;
use colored::*;

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