extern crate cfonts;

use cfonts::{say, BgColors, Colors, Fonts, Options};
use std::{
    collections::HashSet,
    fs::read_to_string,
    io::{self, Write},
    process, vec,
};

/// returns a string of 3 alphabetic characters
/// # Panics
/// Panics stdin read_line fails
fn get_valid_input() -> String {
    loop {
        let mut input = String::new();
        print!("Enter exactly 3 characters e.g. 'abc': ");
        io::stdout().flush().unwrap();

        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read input: {}", err);
            process::exit(1);
        }

        // Remove trailing newline character
        input = input.trim().to_lowercase();

        // Check if input length is 3 and only contains alphabetic characters
        if input.len() == 3 && input.chars().all(|c| c.is_ascii_alphabetic()) {
            return input;
        } else {
            println!("Invalid input. Please enter exactly 3 alphabetic characters.");
        }
    }
}

fn get_valid_letters(char_to_exclude: char, letter_groups: &[HashSet<char>]) -> HashSet<char> {
    let filtered_letters: HashSet<char> = letter_groups
        .iter()
        .filter(|side| !side.contains(&char_to_exclude))
        .flatten()
        .cloned()
        .collect();

    filtered_letters
}

/// Given the vector of letters and vector of words in the dictionary, return valid words.
fn find_valid_words(letter_groups: &[HashSet<char>], letters_set: &HashSet<char>, word_list: &[&str]) -> Vec<String> {
    let mut found_words: Vec<String> = Vec::new();

    // for each word in the dictionary see if it can be spelled with a sequence of valid letters
    for word in word_list {
        // words must be more than 3 characters in the game
        if word.len() < 3 {
            continue;
        }

        let mut valid = true;
        let mut prev_char: Option<char> = None;

        for letter in word.chars() {
            if let Some(prev) = prev_char {
                // check if the current letter is valid from the previous letter
                valid = letters_set.contains(&letter)
                    && get_valid_letters(prev, letter_groups).contains(&letter);
            } else {
                // check if the first letter is valid
                valid = letters_set.contains(&letter);
            }

            if !valid {
                break;
            }

            prev_char = Some(letter);
        }

        if valid {
            found_words.push(word.to_string());
        }
    }

    found_words
}

// given the string of letters and vector of valid words, find sets of two words that include all letters at least once
fn join_words(letters_set: &HashSet<char>, valid_words: &[String]) -> Vec<(String, String)> {
    let mut found_combos: Vec<(String, String)> = Vec::new();

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
            // Create a new HashSet to store the characters from both words
            let mut word_set: HashSet<char> = HashSet::new();
            word_set.extend(first_word.chars());
            word_set.extend(second_word.chars());

            // Check if the word_set contains all the characters from letters_set
            if letters_set.is_subset(&word_set) {
                found_combos.push((first_word.to_string(), second_word.to_string()));
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

    let dictionary = match read_to_string("data/dict_large.txt") {
        Ok(content) => content.to_lowercase(),
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            process::exit(1);
        }
    };

    // check to see if the line has '\n' or '\r\n' then split based on that file ending
    let dictionary_words: Vec<&str> = dictionary
        .split("\r\n")
        .flat_map(|line| line.split('\n'))
        .map(|word| word.trim())
        .collect();

    // letter_groups are shaped like this [('a','b','c'), ('d','e','f'), ('g','h','i'), ('j','k','l')];
    let mut letter_groups: Vec<HashSet<char>> = vec![];
    for _ in 0..4 {
        let letters: HashSet<char> = get_valid_input().chars().collect();
        letter_groups.push(letters.clone());
    }

    let letters_set: HashSet<char> = letter_groups.iter().flatten().cloned().collect();

    print!("letters in the thing: {:?}", letters_set);

    let valid_words: Vec<String> = find_valid_words(&letter_groups, &letters_set, &dictionary_words);

    println!("len of valid_words: {}", valid_words.len());

    let shortest_combinations = join_words(&letters_set, &valid_words);
    println!("Two-word combinations of valid words that include all letters:");
    for combination in shortest_combinations {
        println!("{}->{}", combination.0, combination.1);
    }

    // Check for one-word solutions
    let mut solution_found = false;
    for word in &valid_words {
        let word_set: HashSet<char> = word.chars().collect();
        if letters_set.is_subset(&word_set) {
            solution_found = true;
            println!("WOW! One-word solution: {}", word);
        }
    }
    if !solution_found {
        println!("No One-word solutions found :(");
    }
}
