fn main() {
    println!("Hello, world!");

    let x = 2;
    println!("{}", x);

    let y = x; //changes owner of value to x, and drops y variable. y variable can no longer be refered it.
    //println!("{}", x); // would not compile.

    let z = y.clone(); // creates new value for z, y can still be used. 
    //(a clone might in other languages be refered to as "a deep copy")

}
