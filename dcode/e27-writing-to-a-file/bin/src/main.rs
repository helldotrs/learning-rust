use std::fs::File; //file struct
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
        .expect("could not create file!");

    file.write_all(b"Hello Kitty!")
        .expect("could not write to file!");
}
