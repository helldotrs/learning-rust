fn main() {
    let number = 11;

    match number{ //"switch"
        1           => println!("The one!"),
        2           => println!("TWO!!"),
        10 | 11     => println!("ten or eleven"), //can not use or/OR apparently :(
        _           => println!("no match!") //_ for "else"
    }
    println!("Hello, world!");
}
