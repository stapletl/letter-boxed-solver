use std::{
    fs::read_to_string,
    io::{self, Write},
};

/// returns a string of 3 alphabetic characters
/// # Panics
/// Panics stdin read_line fails
fn get_valid_input() -> String {
    loop {
        let mut input = String::new();
        println!("Enter exactly 3 characters (a-z, A-Z): ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Remove trailing newline character
        input = input.trim().to_string();

        // Check if input length is 3 and only contains alphabetic characters
        if input.len() == 3 && input.chars().all(|c| c.is_ascii_alphabetic()) {
            return input;
        } else {
            println!("Invalid input. Please enter exactly 3 alphabetic characters.");
        }
    }
}

fn main() {
    // print

    let popular_words =
        read_to_string("popular.txt").expect("Something went wrong reading the file");

    let dict_vec: Vec<&str> = popular_words.split('\n').collect();

    let mut letters_vec: Vec<String> = vec![];

    for _ in 0..4 {
        let letters = get_valid_input();
        letters_vec.push(letters.clone());

        println!("your input (converted to string) was: {}", letters);
    }

    // print!("your input was: {}", input);

    // for (index, word) in letters_vec.iter().enumerate() {
    //     println!("Set of letters {}: {}", index, word);
    // }

    // let mut found_words = Vec::new();
    // for word in dic_vec {
    //     if word.len() <= 6 {
    //         let mut temp_user_input = user_input.to_string();
    //         let mut temp_word = word.to_string();
    //         let mut found = true;
    //         for c in word.chars() {
    //             if temp_user_input.contains(c) {
    //                 temp_user_input = temp_user_input.replacen(c, "", 1);
    //                 temp_word = temp_word.replacen(c, "", 1);
    //             } else {
    //                 found = false;
    //                 break;
    //             }
    //         }
    //         if found {
    //             found_words.push(word);
    //         }
    //     }
    // }

    // print all words that can be spelled with the user input
    // println!("Words that can be spelled with the user input:");
    // for word in found_words {
    //     println!("{}", word);
    // }
}
