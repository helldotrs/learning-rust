use std::fs::File; //struct for file
use std::io::prelude::*; //prelude module helps use read and write to a file

fn main() {
    let mut file = File::open("text.txt")
        .expect("Can't open file!");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Can not read file!");

    println!("File content:\n{}", contents);
}
//You can also do --> let contents: String = std::fs::read_to_string("filename.txt");
