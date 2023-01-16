use colored::*;

pub fn first_word(words: &str) -> &str {
  println!("\n{}\n", "First word of a word list".bold().italic().blue());
  for i in 0..words.len() {
    if &words[i..i+1] == " " {return &words[0..i]}
  }
  words
}