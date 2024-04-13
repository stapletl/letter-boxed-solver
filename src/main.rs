extern crate cfonts;

use cfonts::{say, BgColors, Colors, Fonts, Options};
use std::{
    collections::HashSet,
    fs::read_to_string,
    io::{self, Write},
    vec,
};

/// returns a string of 3 alphabetic characters
/// # Panics
/// Panics stdin read_line fails
fn get_valid_input() -> String {
    loop {
        let mut input = String::new();
        println!("Enter exactly 3 characters e.g. 'abc': ");
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

/// Given a character and a vector of letters, return all letters that are not in the same element as it
fn get_valid_letters(char_to_exclude: char, letters: &[String]) -> String {
    let filtered_letters: Vec<&String> = letters
        .iter()
        .filter(|side| !side.contains(char_to_exclude))
        .collect();

    filtered_letters
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Given the vector of letters and vector of words in the dictionary, return valid words.
fn find_valid_words(letters: &[String], dict: &[&str]) -> Vec<String> {
    let letters_string = letters.join("");
    let mut found_words: Vec<String> = Vec::new();

    // for each word in the dictionary see if it can be spelled with a sequence of valid letters
    for word in dict {
        let mut valid = true;
        let mut prev_char: char = '0'; // this is not a valid alphabetic char, so we initialize to it
        for (index, letter) in word.char_indices() {
            // check if first letter is is valid
            if index == 0 {
                if !letters_string.contains(letter) {
                    valid = false;
                }
            } else {
                // otherwise make sure the current letter is valid from the previous letter
                valid = get_valid_letters(letter, &letters).contains(prev_char);
            }

            prev_char = letter;
            if !valid {
                break;
            };
        }
        if valid {
            found_words.push(word.to_string());
        }
    }

    found_words
}

// given the string of letters and vector of valid words, find sets of two words that include all letters at least once
fn join_words(letters: &str, valid_words: &[String]) -> Vec<(String, String)> {
    let mut found_combos: Vec<(String, String)> = Vec::new();

    println!("inside fn join_words");

    for first_word in valid_words {
        let words_that_link: Vec<&String> = valid_words
            .iter()
            .filter(|word_to_link| {
                if let (Some(first_char), Some(last_char)) =
                    (word_to_link.chars().next(), first_word.chars().last())
                {
                    first_char == last_char && word_to_link.len() + first_word.len() >= 12
                } else {
                    false
                }
            })
            .collect();
        for second_word in words_that_link {
            // see if all letters are in the linked word
            let full_word = [first_word.clone(), second_word.clone()].join("");
            // print!("full word: {}", full_word);
            if letters.chars().all(|letter| full_word.contains(letter)) {
                found_combos.push((first_word.to_string(), second_word.to_string()))
            }
        }
    }

    found_combos
}

fn main() {
    // printab
    say(Options {
        text: String::from("LetterBoxed Solver"),
        font: Fonts::Font3d,
        colors: vec![Colors::BlueBright],
        background: BgColors::Transparent,
        max_length: 6,
        ..Options::default()
    });

    let popular_words =
        read_to_string("dict_large.txt").expect("Something went wrong reading the file");

    let dict_vec: Vec<&str> = popular_words.split('\n').map(|word| word.trim()).collect();

    let mut letters_vec: Vec<String> = vec![];

    for _ in 0..4 {
        let letters = get_valid_input();
        letters_vec.push(letters.clone());
    }

    let letters_string = letters_vec.join("");

    let valid_words = find_valid_words(&letters_vec, &dict_vec);

    let shortest_combinations = join_words(&letters_string, &valid_words);
    println!("Shortest combinations of valid words that include all letters:");
    for combination in shortest_combinations {
        println!("{}, {}", combination.0, combination.1);
    }
}
