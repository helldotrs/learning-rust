fn main() {
    println!("Hello, world!");

    let x = 2;
    println!("{}", x);

    let mut y = x; //changes owner of value to x, and drops y variable. y variable can no longer be refered it.
    //println!("{}", x); // would not compile.

    let z = y.clone(); // creates new value for z, y can still be used. 
    //(a clone might in other languages be refered to as "a deep copy")

    y = add_one(y); //once var is sent to func var is dropped: unusuall but valid solution, normally you'd use a value.
    // y == 3;

    

    
}

fn add_one (a: String) -> String{
    a + 1
}


