use std::io;

//rust docs declares functions after main and chat-gpt before. The advantage with declaring them before is that it feels a bit more logical, on the other hand; the advantage of writing them after is that it makes them a bit more abstract and may help when starting with psudocode. 
//I think I will functions after main for now.

fn main() {
    
    println!("please enter value to convert (c-->f & f-->c)");
    let mut input: string = String::new();
    io::stdin().read_line(&mut input).expect("failure to read input");
    // we now have input var with string value from the input.
    
    // now lets convert, or pharse, as a in integer (i32):
    let input: i32 = match input.trim().parse() { //using .trim() every time we parse seems to be convention in rust - but is it really neccecary?
        Ok(n) => n,
        Err(_) => panic!("invalid input"), //sooooo neat that we always test and create a panic! in rust
    }; 
    println!("Hello, world!");

    
}


//non-main-functions:
fn int2float --> f64 (a){
    //FIXME
}

fn c2f --> f64 (a){
    //FIXME
    //(°C x 1.8) + 32
}

fn f2c --> f64 (a){  // FIXME: ref
        a = a - 32;
        a = a * 5;
        a = a / 9;
        a
}


