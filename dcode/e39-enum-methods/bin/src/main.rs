#![allow(dead_code)]

enum Day{
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday

}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday  => return false,
            _                               => return true
        }
    }

}

fn main() {
    let d1 = Day::Tuesday;
    let d2 = Day::Sunday;
    println!("Hello, world!");

    println!("is d1 a weekday? {}", d1.is_weekday());
    println!("is d2 a weekday? {}", d2.is_weekday());
}
