use std::io; //standard in struct or string?

fn main() {
    let mut input = String::new();
    println!("say something");

    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("success, you said: {}", input.to_uppercase());
        }, 
        Err(e) => {
            println!("Error: {}", e);
        }
    }

}
