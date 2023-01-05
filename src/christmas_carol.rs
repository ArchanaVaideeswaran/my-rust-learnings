use colored::*;

pub fn main() {
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