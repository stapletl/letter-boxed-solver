use std::fs::read_to_string;

fn main() {
    let popular_words = read_to_string("popular.txt")
        .expect("Something went wrong reading the file");

    let words_vec: Vec<&str> = popular_words.split("\n").collect();

    // user input list of 6 characters
    let mut user_input = String::new();
    println!("Enter 6 characters: ");
    std::io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.trim();

    let mut found_words = Vec::new();
    for word in words_vec {
        if word.len() <= 6 {
            let mut temp_user_input = user_input.to_string();
            let mut temp_word = word.to_string();
            let mut found = true;
            for c in word.chars() {
                if temp_user_input.contains(c) {
                    temp_user_input = temp_user_input.replacen(c, "", 1);
                    temp_word = temp_word.replacen(c, "", 1);
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                found_words.push(word);
            }
        }
    }

    // print all words that can be spelled with the user input
    println!("Words that can be spelled with the user input:");
    for word in found_words {
        println!("{}", word);
    }


}
