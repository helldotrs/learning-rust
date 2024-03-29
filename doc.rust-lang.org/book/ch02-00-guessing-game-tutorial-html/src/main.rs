//bookmark: expression to move from crashing on an error to handling the error. Remember that
//extern rand;

use rand::Rng;
//was going to add "use std::cmp::Ordering;"" here but the analyzer modified the following line instead:
use std::{io, cmp::Ordering}; // io --> input/output


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        /*
            The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
        */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //u32 --> small positive number.
        //Result [type] is an enum that has the variants Ok and Err
        //.parse() here turns the String into a u32 and returns Ok, or Err

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number){ //The cmp method compares two values and can be called on anything that can be compared. 
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("U r winrar!");
                break;
            },
        } //the Ordering type is another enum and has the variants Less, Greater, and Equal
        //The match expression ends after the first successful match
    }


}
