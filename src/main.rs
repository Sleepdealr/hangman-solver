use regex::Regex;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
//TODO: Create regex generator to filter non-possible words
//TODO: Fix output 

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_most_common_letter(list: &mut Vec<String>, used_letters: &str) -> char {
    let mut letter_map: HashMap<char, u32> = HashMap::new();
    for word in list.into_iter() {
        for (letter) in word.chars() {
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

fn prune_vec(words: &mut Vec<String>, incorrect_letters: &str) {
    if incorrect_letters.is_empty() {
        return;
    }
    let formatted = format!("/[{}]/", incorrect_letters);
    let incorrect = Regex::new(&*formatted).unwrap();
    for word in words {}
}

fn game(secret_word: String) {
    let (mut incorrect_letters, mut used_letters): (String, String) =
        (String::new(), String::new());
    let mut cur_guess: String = "_".repeat(secret_word.len());
    let mut attempts: u8 = 0; //Attempts counter
    let mut letter_guess: char = ' ';
    let mut words: Vec<String> =
        read_file("english-words/words_lowercase.txt").expect("Could not load lines"); // Loads all lines from file into words
    words.retain(|x| x.len() == secret_word.len()); // Prunes all words that aren't the correct length

    loop {
        // Game loop
        letter_guess = get_most_common_letter(&mut words, &used_letters);

        let mut temp_cur_guess: String = String::new();
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
        println!(
            "Most common letter is:{}\nCurGuess:{}",
            letter_guess, cur_guess
        );
        if secret_word.eq(&cur_guess) {
            println!("it worky! {}", cur_guess);
            break;
        }
        prune_vec(&mut words, &*incorrect_letters);
    }
    println!(
        "Most common letter: {}",
        get_most_common_letter(&mut words, &used_letters)
    );
    println!("{:?}", words);
}

fn main() {
    let mut secret_word = String::new();
    println!("What word do you want it to guess?");
    std::io::stdin().read_line(&mut secret_word).unwrap(); // Gets user input from stdin
    secret_word = secret_word.trim().to_string(); // Remove \n and other gunk from end of word
    game(secret_word);
}
