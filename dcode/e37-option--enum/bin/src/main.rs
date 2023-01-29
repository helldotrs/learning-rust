fn main() {
    
    let name        = String::from("Hellmak");

    
    println!("Char at index 8: {}", match name.chars().nth(8){ //chars return iterator with chars, nth get item at passed index
        Some(a) => a.to_string(),
        None    => "no character".to_string()
    });
    
    println!("Char at index 8: {}", match name.chars().nth(0){ //chars return iterator with chars, nth get item at passed index
        Some(a) => a.to_string(),
        None    => "no character".to_string()
    });
    


}
