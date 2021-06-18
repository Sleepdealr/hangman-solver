use regex::Regex;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    time::Instant,
};
//TODO: Fix output

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_most_common_letter(list: &mut Vec<String>, used_letters: &str) -> char {
    let mut letter_map: HashMap<char, u32> = HashMap::new();
    for word in list.into_iter() {
        for letter in word.chars() {
            // Gets index(unused) and letter of each word
            *letter_map.entry(letter).or_insert(1) += 1; // Adds letter to map if it doesn't exit
        }
    }
    let mut mcl: char = ' ';
    let mut mcl_val = 0;
    for l in letter_map {
        if l.1 > mcl_val && !used_letters.contains(l.0) {
            mcl = l.0;
            mcl_val = l.1;
        }
    }
    mcl
}

fn prune_vec(words: &mut Vec<String>, incorrect_letters: &str, cur_guess: &str) {
    let formatted;
    let replaced = cur_guess.replace("_" , "[a-z]{1}");
    let reg_replaced = Regex::new(&replaced).unwrap();
    if !incorrect_letters.is_empty() {
        formatted = format!("[{}]", incorrect_letters);
        let incorrect = Regex::new(&formatted).unwrap();
        words.retain(|x| !incorrect.is_match(x) && reg_replaced.is_match(x));
        return
    }
    words.retain(|x| reg_replaced.is_match(x));
}

fn game(secret_word: String) {
    let now = Instant::now();
    let (mut incorrect_letters, mut used_letters): (String, String) =
        (String::new(), String::new());
    let mut cur_guess: String = "_".repeat(secret_word.len());
    let mut attempts: u8 = 1; //Attempts counter
    let mut letter_guess: char;
    let mut words: Vec<String> =
        read_file("english-words/words_lowercase.txt").expect("Could not load lines"); // Loads all lines from file into words
    words.retain(|x| x.len() == secret_word.len()); // Prunes all words that aren't the correct length

    loop {
        // Game loop
        letter_guess = get_most_common_letter(&mut words, &used_letters);

        let mut temp_cur_guess: String = String::new();
        if !secret_word.contains(letter_guess) {
            incorrect_letters.push(letter_guess);
        }
        for (index, letter) in secret_word.chars().enumerate() {
            if letter.eq(&letter_guess) {
                temp_cur_guess.push(letter);
            } else if !(cur_guess.chars().nth(index).unwrap() == '_') {
                temp_cur_guess.push(cur_guess.chars().nth(index).unwrap());
            } else {
                temp_cur_guess.push('_');
            }
        }

        cur_guess = temp_cur_guess;
        used_letters.push(letter_guess);

        dbg!(&cur_guess);
        dbg!(&letter_guess);

        if secret_word.eq(&cur_guess) {
            println!("it worky! {}", cur_guess);
            break;
        }

        prune_vec(&mut words, &incorrect_letters, &cur_guess);
        attempts += 1;
    }
    println!("This took {} milliseconds", now.elapsed().as_millis());
    dbg!(&attempts);
}

fn main() {
    let mut secret_word = String::new();
    println!("enter a word it will guess");
    std::io::stdin().read_line(&mut secret_word).unwrap(); // Gets user input from stdin
    secret_word = secret_word.trim().to_string(); // Remove \n and other gunk from end of word
    game(secret_word);
}
