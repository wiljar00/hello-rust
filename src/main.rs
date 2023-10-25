// first hello world project: 
// use ferris_says::say; 
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(&message, width, &mut writer).unwrap();
// }



// multi function app to read in an input and tell you if RUST can be built from the input characters. 

use ferris_says::say; 
use std::io::{stdout, BufWriter};
use std::collections::HashMap;

fn can_build_rust(input: &str) -> bool {
    let mut char_counts = HashMap::new();

    // Count the occurrences of each character in the input string
    for ch in input.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }

    // Check if the characters required for "rust" are present
    let required_chars: HashMap<char, usize> = [('r', 1), ('u', 1), ('s', 1), ('t', 1)]
        .iter()
        .cloned()
        .collect();

    // Check if all the required characters are present with sufficient count
    required_chars
        .iter()
        .all(|(ch, count)| char_counts.get(ch).unwrap_or(&0) >= count)
}

fn have_ferris_answer(input: &str) {
    let stdout = stdout();
    let message = String::from(input);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn main() {
    println!("Please enter a string and I will tell you if it is rusty (contains characters R.U.S.T.): ");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result = can_build_rust(input.trim());

    if result {
        have_ferris_answer("You can build 'rust' from the entered characters.");
    } else {
        have_ferris_answer("You cannot build 'rust' from the entered characters.");
    }
}
