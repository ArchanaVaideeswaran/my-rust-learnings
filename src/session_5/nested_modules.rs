use colored::*;

pub mod movies {
  pub mod english {
     pub mod comedy {
        pub fn play(name:String) {
           println!("Playing comedy movie {}",name);
        }
     }
  }
}
use movies::english::comedy::play; 
// importing a public module

pub fn main() {
  println!("\n{}\n", "Rust Nested Modules".bold().italic().blue());
  // short path syntax
  play("Herold and Kumar".to_string());
  play("The Hangover".to_string());

  //full path syntax
  movies::english::comedy::play("Airplane!".to_string());
}
