// Nikhil Yachareni - 05/25/2022
use std::io;

fn main() {
    let mut english = String::new();
    let mut pig_latin = String::new();

    io::stdin()
        .read_line(&mut english)
        .expect("Failed to read input line!");

    for word in english.split_whitespace() {
        // First, convert word to a &str so we can slice
        let word_slice: &str = &word[..];  // full slice

        let mut first_char = String::new();
        let mut rest = String::new();

        // Separate word_slice into first_char and rest
        // Iterate over first 5 bytes to get Unicode scalar value
        for i in 1..5 {
            let temp_char = &word_slice.get(0..i).map(str::to_string);
            match temp_char {
                // slice only if valid char is obtained
                Some(x) => {
                    first_char = x.to_string();
                    
                    // check if vowel and if yes, add -hay instead
                    if first_char == "a" || first_char == "e" 
                    || first_char == "i" || first_char == "o" 
                    || first_char == "u" {
                        rest = word_slice[..].to_owned();
                        pig_latin.push_str(&rest);
                        pig_latin.push_str("-hay ")
                    }
                    else { // for consonant
                        rest = word_slice[i..].to_owned();
                        pig_latin.push_str(&rest);
                        pig_latin.push_str("-");
                        pig_latin.push_str(&first_char);
                        pig_latin.push_str("ay ");
                    }
                    break;
                }, 
                None => (),
            }
        }
    }
    println!("");
    println!("Pig Latin -> {}", pig_latin);
    println!("");
}
