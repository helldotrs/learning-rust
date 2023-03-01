//bookmark: Conditional Loops with while

fn main() {
    //Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable:
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    
    //the return value has to be the same type, hence following wont compile:
    //let number = if condition { 5 } else { "six" };
    
    
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; //breaks loop AND returns value
        }
    };
    println!("The result is {result}");
    
    //loop label
    let mut count = 0;
    'counting_up: loop {  //loop labels begin with '
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; //breaks outer loop / loop with label 'counting up.
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    for element in a { // faster than while < loop, and no risk of wrong value which cause panic!
        println!("the value is: {element}");
    }
    }
    //The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
    //Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a while loop in Listing 3-3, most Rustaceans would use a for loop. 
    
    
}
