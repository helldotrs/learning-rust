fn main() {
    
    let name        = String::from("Hellmak");
    let name_long   = String::from("HellmakHellmak");

    
    println!("Char at index 8: {}", match name.chars().nth(8){ //chars return iterator with chars, nth get item at passed index
        Some(a) => a.to_string(),
        None    => "no character".to_string()
    });
    
    println!("Char at index 8: {}", match name_long.chars().nth(8){ //chars return iterator with chars, nth get item at passed index
        Some(a) => a.to_string(),
        None    => "no character".to_string()
    });
    


}
