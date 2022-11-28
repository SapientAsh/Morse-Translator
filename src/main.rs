use std::collections::HashMap;
use std::io;

fn main() {
    let mut morse = HashMap::<String, String>::new();
    morse.insert(
        ".-".to_string(),
        "A".to_string(),
    );
    morse.insert(
        "-...".to_string(),
        "B".to_string(),
    );
    morse.insert(
        "-.-.".to_string(),
        "C".to_string(),
    );
    morse.insert(
        "-..".to_string(),
        "D".to_string(),
    );
    morse.insert(
        ".".to_string(),
        "E".to_string(),
    );
    morse.insert(
        "..-.".to_string(),
        "F".to_string(),
    );
    morse.insert(
        "--.".to_string(),
        "G".to_string(),
    );
    morse.insert(
        "....".to_string(),
        "H".to_string(),
    );
    morse.insert(
        "..".to_string(),
        "I".to_string(),
    );
    morse.insert(
        ".---".to_string(),
        "J".to_string(),
    );
    morse.insert(
        "-.-".to_string(),
        "K".to_string(),
    );
    morse.insert(
        ".-..".to_string(),
        "L".to_string(),
    );
    morse.insert(
        "--".to_string(),
        "M".to_string(),
    );
    morse.insert(
        "-.".to_string(),
        "N".to_string(),
    );
    morse.insert(
        "---".to_string(),
        "O".to_string(),
    );
    morse.insert(
        ".--.".to_string(),
        "P".to_string(),
    );
    morse.insert(
        "--.-".to_string(),
        "Q".to_string(),
    );
    morse.insert(
        ".-.".to_string(),
        "R".to_string(),
    );
    morse.insert(
        "...".to_string(),
        "S".to_string(),
    );
    morse.insert(
        "-".to_string(),
        "T".to_string(),
    );
    morse.insert(
        "..-".to_string(),
        "U".to_string(),
    );
    morse.insert(
        "...-".to_string(),
        "V".to_string(),
    );
    morse.insert(
        ".--".to_string(),
        "W".to_string(),
    );
    morse.insert(
        "-..-".to_string(),
        "X".to_string(),
    );
    morse.insert(
        "-.--".to_string(),
        "Y".to_string(),
    );
    morse.insert(
        "--..".to_string(),
        "Z".to_string(),
    );
    morse.insert(
        "/".to_string(),
        " ".to_string(),
    );
    morse.insert(
        ".----".to_string(),
        "1".to_string(),
    );
    morse.insert(
        "..---".to_string(),
        "2".to_string(),
    );
    morse.insert(
        "...--".to_string(),
        "3".to_string(),
    );
    morse.insert(
        "....-".to_string(),
        "4".to_string(),
    );
    morse.insert(
        ".....".to_string(),
        "5".to_string(),
    );
    morse.insert(
        "-....".to_string(),
        "6".to_string(),
    );
    morse.insert(
        "--...".to_string(),
        "7".to_string(),
    );
    morse.insert(
        "---..".to_string(),
        "8".to_string(),
    );
    morse.insert(
        "----.".to_string(),
        "9".to_string(),
    );
    morse.insert(
        "-----".to_string(),
        "0".to_string(),
    );
    morse.insert(
        "..--..".to_string(),
        "?".to_string(),
    );
    morse.insert(
        "-.-.--".to_string(),
        "!".to_string(),
    );
    morse.insert(
        ".-.-.-".to_string(),
        ".".to_string(),
    );
    morse.insert(
        "--..--".to_string(),
        ",".to_string(),
    );

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
        output += morse.get(letter).unwrap_or(&String::from(""));
    }

    println!("\n  -> {output}\n");
    }
}