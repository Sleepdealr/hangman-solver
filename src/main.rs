use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use std::collections::HashMap;
use regex::Regex;

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

fn prune_vec(words: &mut Vec<String> , incorrect_letters:&str){
    let formatted = format!("/[{}]/" , incorrect_letters);
    let incorrect = Regex::new(&*formatted).unwrap();
    words.retain(|x|  incorrect.is_match(&x) );
}

fn game(mut secret_word:String){
    let (mut incorrect_letters , mut cur_guess , mut attempts, mut guess): (String, String, u8, char) = (String::new(), "_".repeat(secret_word.len()), 0 , ' ');

    let mut words: Vec<String> = read_file("english-words/words_lowercase.txt").expect("Could not load lines"); // Loads all lines from file into words
    words.retain(|x  | x.len() == secret_word.len()); // Prunes all words that aren't the correct length

    loop{ // Game loop
        guess = get_most_common_letter(&mut words);

        if secret_word.eq(&cur_guess) {
            break;
        }
    }

    prune_vec(&mut words, &*incorrect_letters);

    println!("{:?}" , words);
    println!("Most common letter: {}" , get_most_common_letter(&mut words) )
}

fn main(){
    let mut secret_word = String::new();
    println!("What word do you want it to guess?");
    std::io::stdin().read_line(&mut secret_word).unwrap(); // Gets user input from stdin
    secret_word = secret_word.trim().to_string(); // Remove \n and other gunk from end of word
    game(secret_word);

}