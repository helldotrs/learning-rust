extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"\w{5}").unwrap(); //r"" --> raw string. \d would have to be written \\r in regular string
    println!("Hello, world!");
    let text1 = "dcode";
    let text2 = "Rust";

    println!("found match? {}\nfound match? {}", re.is_match(text1), re.is_match(text2))
}
