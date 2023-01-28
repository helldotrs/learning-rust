extern crate rand;
use rand::Rng;

fn main() {

    let mut num1    = 0;
    let mut num2    = 0;
    let mut num3    = 0;

    let mut i       = 0;
    while i < 1000000 { // 1 000 000
        match rand::thread_rng().gen_range(1..4){
            1 => num1 = num1 + 1,
            2 => num2 = num2 + 1,
            3 => num3 = num3 + 1,
            _ => break
        }

        println!("{}, {}, {}", num1, num2, num3);
        i = i + 1;
    }
    println!("final:");
    println!("{}, {}, {}", num1, num2, num3);
}
