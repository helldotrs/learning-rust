use std::Fs::File;
use std::io::prelude::*;

const file_name: String = String::from("words.txt");




fn main() {
    let selected_word = select_word();
    println!("Selected word was: {}", selected_word);
}




fn select_word() -> String {
    // open file
    let mut file = File::open(file_name)
        .expect("Could not open {}", file_name);

    // load file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Error reading file.");

    // get individual words  // "string literal"
    let available_words: Vec<&str> = file_contents
        .trim()
        .split(",")
        .collect();

    // generate random index
    let random_index = rand::thread_rng()
        .gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
    
}
