use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
    sync::Mutex
};
// use regex::Regex;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn prune_list(){
    //length
}


//how to send mutatable vec
fn asdasd(asd: &mut Vec<String>){
    asd[0] = "str".parse().unwrap();

}

fn main() {
    let mut words  = lines_from_file("english-words/words_lowercase.txt").expect("Could not load lines");
    asdasd(&mut words);
    println!("{}" , words[0]);
}