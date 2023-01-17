use colored::*;
use std::collections::HashMap;

pub fn main() {
    println!("\n{}\n", "Hashmaps in Rust".bold().italic().blue());

    let mut state_codes = HashMap::new();
    state_codes.insert("KL", "Kerala");
    state_codes.insert("MH", "Maharashtra");
    println!("{:?}", state_codes);
    println!("size of map is {}", state_codes.len());

    match state_codes.get(&"KL") {
        Some(value) => {
            println!("Value for key KL is {}", value);
        }
        None => {
            println!("nothing found");
        }
    }

    for (key, val) in state_codes.iter() {
        println!("key: {} val: {}", key, val);
    }

    // if state_codes.contains_key(&"MH") {
    //     println!("found key");
    // }

    match state_codes.contains_key(&"MH") {
        true => println!("found key"),
        false => println!("key not found"),
    }
    
    state_codes.remove(&"KL");
    println!("after removing {:?}", state_codes);
}
