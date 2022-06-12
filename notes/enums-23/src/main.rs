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

    if let Some(x) = my_variable {
        println!("value is {}", x);
    }

    match my_variable {
        Some(x) => {
            println!("Value is {}", x);
        },
        None => {
            println!("no value");
        },
    }

    //Option & Result
    //Option
    //let mut x: Option<i32> = None;
    let mut x = None;
    x = Some(5);
    x.is_some(); // true
    x.is_none(); // false
    for i in x {
        println!("{}", i) //prints 5
    }
    //Result
    #[must_use]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
