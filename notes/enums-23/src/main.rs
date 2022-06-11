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

    use DispenserItem::*;
    let item = Empty; // let item = Ammo(69); //let item = Things("hat".to_string(), 7); // Place {x: 24, y: 48} 

    enum Option<T> {
        Some(t),
        None,
    }

    if let Some(x) = my_variable { //"if let" is handy if you care about one variant matches or not, but not if you want to handle all the variables at once
        println!("value is {}", x);
    }

    match my_variable { //"match" handy when you want to handle all the variants at once
        Some(x) => {  // => is called a double arrow
            println!("Value is {}", x);
        },
        None => {
            println!("no value");
        },
    }
}
