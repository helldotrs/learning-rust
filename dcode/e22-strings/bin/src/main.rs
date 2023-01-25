fn main() {
    let my_string = String::from("How is it going?");

    println!("length: {}", my_string.len());
    println!("string empty: {}", my_string.is_empty());

    
    for token in my_string.split_whitespace(){
        println!("{}", token);
    }

    println!("String contains \"it\": {}", my_string.contains("it"));
    println!("String contains \"hey\": {}", my_string.contains("hey"));
}
