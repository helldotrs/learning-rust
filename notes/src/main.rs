fn main() {
    println!("Hello, world!");

    let x = 2;
    let y = x; //changes owner of value to x, and drops y variable. y variable can no longer be refered it.
}
