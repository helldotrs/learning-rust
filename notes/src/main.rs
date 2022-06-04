fn main() {
    println!("Hello, world!");

    let x = 2;
    println!("{}", x);

    let mut y = x; //changes owner of value to x, and drops y variable. y variable can no longer be refered it.
    //println!("{}", x); // would not compile.

    let z = y.clone(); // creates new value for z, y can still be used. 
    //(a clone might in other languages be refered to as "a deep copy")

    y = add_one(y); //once var is sent to func var is dropped: unusuall but valid solution, normally you'd use a value.
    // y --> 3;

    y = add_two(&y); //& means we pass a reference, rather than ownership of value 
    // y --> 5   

    //Lifetimes --> references must always be valid --> compiler wont let you create a reference that will outlive the data it is referencing & you can never point to Null

    y = add_three(&mut y); //passes mutable reference
    
}

fn add_one (a: String) -> String{
    a + 1
}

fn add_two (a: &String){ //reference is immutable even two value it reference to is mutable.
    y + 2
}

fn add_three(a: &mut String){
    y + 3
}
