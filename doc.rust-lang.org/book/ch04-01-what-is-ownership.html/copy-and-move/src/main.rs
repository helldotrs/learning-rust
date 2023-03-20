
fn main() {
        let s1 = String::from("Hello");
        let i1 = 12;
        println!("{} {}", s1, i1);

        let i2 = i1; //value copied
        println!("{} {}", i1, i2); //compiles

        let s2 = s1; //pointer moved, pointer at s1 dropped, 
        println!("{} {}", s1, s2); //wont compile
}
