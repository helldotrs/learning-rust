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

    println!("Hellmak is a {}", match get_occupation("Hellmak"){
        Some(a) =>  a,
        None    => "No occupation found"
    });
    


}

fn get_occupation(name: &str) -> Option<&str>{
    match name {
        "Hellmak"   => Some("Rustaceans"),
        "Nancy"     => Some("Java dev"),
        _           => None
    }

}
