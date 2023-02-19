

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
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
}
