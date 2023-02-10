//bookmark: method will only work on characters that can logically be converted into numbers and so can easily cause errors. If, for example, the string contained 
//extern rand;

use rand::Rng;
//was going to add "use std::cmp::Ordering;"" here but the analyzer modified the following line instead:
use std::{io, cmp::Ordering}; // io --> input/output


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        /*
            The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string. You’ll find a new function on many types because it’s a common name for a function that makes a new value of some kind.
        */

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        //u32 --> small positive number.

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
