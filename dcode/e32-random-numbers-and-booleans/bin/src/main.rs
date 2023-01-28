extern crate rand;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1,11); //1-10
    println!("random number: {}", random_number);
}
