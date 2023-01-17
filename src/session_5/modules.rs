use colored::*;

pub mod movies {
  pub fn play(name:String) {
     println!("Playing movie {}",name);
  }
}

use movies::play;

pub fn main(){
  println!("\n{}\n", "Rust Modules".bold().italic().blue());
  //movies::play("Herold and Kumar".to_string());
  play("Herold and Kumar".to_string());
}
