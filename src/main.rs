use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use std::collections::HashMap;

// use regex::Regex;

fn read_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn get_most_common_letter(list: &mut Vec<String>) -> char {
    let mut letter_map:HashMap<char , u32> = HashMap::new();
    for word in list.into_iter(){
        for (_index , letter) in word.chars().enumerate(){ // Gets index(unused) and letter of each word
            *letter_map.entry(letter).or_insert(1) += 1; // Adds letter to map if it doesn't exit
        }
    }
    let mut mcl:char = ' ';
    let mut mcl_val = 0;
    for l in letter_map{
        if l.1 > mcl_val {
            mcl = l.0;
            mcl_val = l.1;
        }
    }
    mcl
}

fn main(){
    let mut words: Vec<String> = read_file("english-words/words_lowercase.txt").expect("Could not load lines"); // Loads all lines from file into words
    println!("What word do you want it to guess?");
    let mut secret_word = String::new();
    std::io::stdin().read_line(&mut secret_word).unwrap(); // Gets user input from stdin
    secret_word.pop(); // Remove \n from word
    words.retain(|x  | x.len() == secret_word.len()); // Prunes all words that aren't the correct length
    println!("{:?}" , words);
    println!("Most common letter: {}" , get_most_common_letter(&mut words) )
}