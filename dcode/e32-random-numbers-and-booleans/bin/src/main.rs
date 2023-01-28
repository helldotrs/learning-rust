extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..11); //1-10
    println!("random number: {}", random_number);

    let random_boolean = rand::thread_rng().gen_bool(1.0/2.0); //passing 2 means it has 1 in 2 chance of turning out true.
    println!("coin toss: {}", random_boolean)
}
