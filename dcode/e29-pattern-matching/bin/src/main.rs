fn main() {
    let number = 9;

    match number{ //"switch"
        1           => println!("The one!"),
        2..=9       => println!("TWO-nine!!"), // ..= for inclusive range.   
                                               // ... gives: rustc --explain E0658
                                               // unstable feature, only works in nightly build, not in beta/stable
        10 | 11     => println!("ten or eleven"), //can not use or/OR apparently :(
        _           => println!("no match!") //_ for "else"
    }
    println!("Hello, world!");
}
