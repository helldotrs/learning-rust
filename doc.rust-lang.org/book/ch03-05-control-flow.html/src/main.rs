

fn main() {
    //Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable:
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    
    //the return value has to be the same type, hence this wont compile:
    //let number = if condition { 5 } else { "six" };
}
