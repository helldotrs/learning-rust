// hash maps --> collection of key value pairs
//key => value
use std::collections::HashMap;

fn main() {
    let mut grades = HashMap::new();

    grades.insert("Rust Programming", 96);
    grades.insert("Python Programming", 94);
    grades.insert("Bash Programming", 92);

    println!("number of subjects: {}", grades.len());
}
