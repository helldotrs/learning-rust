fn main() {
    println!("Hello, world!");
    //more like Algebraic Data  Types in Haskall than C-like enums
    //"like a union in C only so much better"

    enum Color {
        Red,
        Green,
        Blue,
    }

    //let color = Color::Red();

    enum DispenserItem {
        Empty,
        Ammo(u8),
        Things(String, i32),
        Place {x: i32, y: i32},
    }
}
