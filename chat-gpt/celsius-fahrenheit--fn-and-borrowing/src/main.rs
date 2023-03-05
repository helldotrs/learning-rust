use std::io;

// This function takes a reference to a temperature in Celsius as input
// and returns the corresponding temperature in Fahrenheit
fn celsius_to_fahrenheit(celsius: &f32) -> f32 {
    // Here, we are borrowing the value of celsius, rather than taking
    // ownership of it. This means that the calling code retains ownership
    // of the value, and we can only read its value without modifying it.
    // This function returns a new value, which does not affect the original
    // value of celsius.
    celsius * 1.8 + 32.0
}

// This function takes a reference to a temperature in Fahrenheit as input
// and returns the corresponding temperature in Celsius
fn fahrenheit_to_celsius(fahrenheit: &f32) -> f32 {
    // Here, we are again borrowing the value of fahrenheit, rather than
    // taking ownership of it. This means that the calling code retains
    // ownership of the value, and we can only read its value without
    // modifying it. This function returns a new value, which does not affect
    // the original value of fahrenheit.
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
            // Here, we are passing a reference to the value of celsius to
            // the celsius_to_fahrenheit function. This means that we are
            // borrowing the value of celsius and passing it to the function
            // for the duration of the function call. The function can read
            // the value of celsius, but it cannot modify it, and it cannot
            // outlive the function call.

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
           

