extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"\w{5}").unwrap(); //r"" --> raw string. \d would have to be written \\r in regular string
    println!("Hello, world!");
    let text1 = "dcode";
    let text2 = "Rust";

    println!("found match? {}\nfound match? {}", re.is_match(text1), re.is_match(text2));

    
    let re2 = Regex::new(r"(\w{5})").unwrap();


    print!("text2: ");
    match re2.captures(text2){
        Some(a) => println!("match found: {}",a.get(0).unwrap().as_str()),
        None    => println!("no match")


    }

    print!("text1: ");
    match re2.captures(text1){
        Some(a) => println!("match found: {}",a.get(0).unwrap().as_str()),
        None    => println!("no match")


    }
    let re2 = Regex::new(r"(\w{5})").unwrap();

    

    print!("text1: ");
    match re2.captures(text1){
        Some(a) => println!("match found: {}", &a[0]), //does same but less verbose
        None    => println!("no match")


    }

}
