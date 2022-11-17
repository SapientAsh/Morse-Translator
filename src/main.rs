use std::collections::HashMap;
use std::io;
use std::fs;

fn main() {
    let mut morse = HashMap::<&str, &str>::new();
     
    let contents = fs::read_to_string("morse.json").expect("Could not read morse data");
    let parsed = json::parse(contents.as_str()).unwrap();

    for i in 0..parsed["Letters"].len() {
        let letter = &parsed["Letters"][i];
        morse.insert(
            letter["morse"].as_str().unwrap(),
            letter["alpha"].as_str().unwrap());
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
        output += morse.get(letter).unwrap_or(&"#");
    }

    println!("\n  -> {output}\n");
    }
}
