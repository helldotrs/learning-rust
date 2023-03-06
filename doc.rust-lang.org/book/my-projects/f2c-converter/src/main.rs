use std::io;

//rust docs declares functions after main and chat-gpt before. The advantage with declaring them before is that it feels a bit more logical, on the other hand; the advantage of writing them after is that it makes them a bit more abstract and may help when starting with psudocode. 
//I think I will functions after main for now.

fn main() {
    
    println!("please enter value to convert (c-->f & f-->c)");
    let mut input: string = String::new();
    io::stdin().read_line(&mut input).expect("failure to read input");
    
    println!("Hello, world!");

    fn f2c --> f64 (a){  // FIXME: ref
        a = a - 32;
        a = a * 5;
        a = a / 9;
        a
    }

    //FIXME: c2f
    //fixme: take input
    

}
