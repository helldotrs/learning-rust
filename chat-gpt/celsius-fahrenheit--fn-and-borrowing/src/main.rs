/* Code from ChatGPT, own comments will be in /// or  /** **/
*/

/*
Writte me the source code for a celsius fahrenheit converter
that works both ways written in Rust that is well commented and
that uses functions and borrowing. it is to help me udnerstand Rust concepts*/

use std::io;

// This function takes a temperature in Celsius as input and returns the corresponding temperature in Fahrenheit
fn celsius_to_fahrenheit(celsius: &f32) -> f32 {
    celsius * 1.8 + 32.0
}

// This function takes a temperature in Fahrenheit as input and returns the corresponding temperature in Celsius
fn fahrenheit_to_celsius(fahrenheit: &f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn main() {
    // Prompt the user to choose which conversion to perform
    println!("Which conversion do you want to perform?");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    // Read the user's choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    // Parse the user's choice as an integer
    let choice: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    // Check the user's choice and perform the corresponding conversion
    match choice {
        1 => {
            // Prompt the user to enter a temperature in Celsius
            println!("Enter the temperature in Celsius:");

            // Read the user's input
            let mut celsius_input = String::new();
            io::stdin().read_line(&mut celsius_input).expect("Failed to read line");

            // Parse the user's input as a float
            let celsius: f32 = match celsius_input.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Invalid input"),
            };

            // Call the celsius_to_fahrenheit function to convert the temperature
            let fahrenheit = celsius_to_fahrenheit(&celsius);

            // Print the result
            println!("{}°C is equivalent to {}°F", celsius, fahrenheit);
        }
        2 => {
            // Prompt the user to enter a temperature in Fahrenheit
            println!("Enter the temperature in Fahrenheit:");

            // Read the user's input
            let mut fahrenheit_input = String::new();
            io::stdin().read_line(&mut fahrenheit_input).expect("Failed to read line");

            // Parse the user's input as a float
            let fahrenheit: f32 = match fahrenheit_input.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Invalid input"),
            };

            // Call the fahrenheit_to_celsius function to convert the temperature
            let celsius = fahrenheit_to_celsius(&fahrenheit);

            // Print the result
            println!("{}°F is equivalent to {}°C", fahrenheit, celsius);
        }
        _ => {
            println!("Invalid choice. Please enter 1 or 2.");
        }
    }
}

/*
In this implementation, we define two functions: celsius_to_fahrenheit and fahrenheit_to_celsius. Both functions take a reference to a float as input (i.e., they borrow the value rather than taking ownership of it), and they return a float. These functions use the formulas for converting between Celsius and Fahrenheit temperatures.

In the main function, we prompt the user to choose which conversion they want to perform. We read the user's choice, parse it as an integer, and use a match expression to perform the corresponding conversion. In each case, we prompt the user to enter a temperature, read their input

*/
