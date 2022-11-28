use serde::Deserialize;

use std::collections::HashMap;
use std::{io, fs};

#[derive(Deserialize)]
struct MorseKey {
    letters: Vec<MorsePair>,
}

#[derive(Deserialize)]
struct MorsePair {
    morse: String,
    alpha: String,
}

fn main() {
    let contents = fs::read_to_string("morse.json").expect("Could not read morse.json");
    let parsed: MorseKey = serde_json::from_str(&contents).unwrap();

    let mut morse = HashMap::<&str, &str>::new();
    for letter in parsed.letters.iter() {
        morse.insert(
            &letter.morse,
            &letter.alpha,
        );
    }

    loop {
    println!("Enter your Morse Code for translation or \"exit\" to exit:");

    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Couldn't read line");
    if response.trim() == "exit" {
        break;
    }

    let letters: Vec<&str> = response.trim().split(" ").collect();

    let mut output: String = String::from("");
    for letter in letters {
        output += morse.get(letter).unwrap_or(&"");
    }

    println!("\n  -> {output}\n");
    }
}