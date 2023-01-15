enum Direction{
    Up,
    Down,
    Left, 
    Right
}

const MAXIMUM_NUMBER: u8 = 20; //should be global? //constants need to have type declared 

fn main() {

    // what is used for % in Rust?
    //single line comment
    /*
    multi line comment
    */

    let mut a: i32  = 45;    // i32
                            // u64 for "unsigned 64 bit integer" does not support negative numbers
    let _b          = 4.5;  // f32 - f for float
    let _c: bool    = true; // boolean // _ for unused variable
    let test_value  = 20;
    let tv          = &test_value; //fixme - come back to this.
    let mut i       = 0;
    let animals     = vec!["Rabbit", "Shark", "Cat"];


    println!("Hello, world!");
    println!("The value of a is {}", a);

    a = 65;

    println!("The value of a is {}", a);

    if a > test_value {
        println!("a is more than {}", test_value);
    }
    else if a == 40 {
        println!("a is {}", test_value);
    }
    else {
        println!("a is less than {}", test_value)
    }

    loop {
        i += 1;

        if i == 10 {
            continue;
        }
        
        if i > test_value {
            break;
        }

        println!("Value of i is {}", i);

    }
    i = 0;

    while i <= test_value {
        i += 1;


        if i % 5 == 0 {
            println!("{}", i);
        }

        // can use break and continue in while loop

        
    }
    i = 0;

    for i in 1..11 {
        println!("The number is {}", i);
    }

    let test_range = 30..51; //non-exclusive, hence 30-50

    for i in  test_range{
        println!("range {}", i);
    }
    
    println!("global i var is still {}", i);

    for (index, a) in animals.iter().enumerate() {//local a for animals  //using iter() method prevents ownership of the vector to being moved to the for-loop //enumerate() to find out index values?
        println!("{} is an animal, index: {}", a, index);
    }

    let player_direction:Direction = Direction::Up; //variable_name:Type (enum) = Type::varient (of type);

    match player_direction{ //match works like switch in other languages
        Direction::Up       => println!("we are heading up!"),
        Direction::Down     => println!("we are heading down!"),
        Direction::Left     => println!("we are heading left!"),
        Direction::Right    => println!("we are heading right!"),

    }

    let player_direction:Direction = Direction::Down; //variable_name:Type (enum) = Type::varient (of type);

    match player_direction{ //match works like switch in other languages
        Direction::Up       => println!("we are heading up!"),
        Direction::Down     => println!("we are heading down!"),
        Direction::Left     => println!("we are heading left!"),
        Direction::Right    => println!("we are heading right!"),

    }

    for n in 1..MAXIMUM_NUMBER{ //non-inclusive, ea 1-19
        println!("{}", n);
    }

    let tup1 = (20, 25, 30, 35, "Rust", 3.14, false, (1, 4, 7)); //tuple //can hold different value types
    let tup2 = (10, 20, 30);
    let (mik, mac, muk) = tup2;

    println!("tup1 first value: {}", tup1.0); //prints first value of tuple tup1, ea 20
    println!("tup1 7th value: {}", tup1.6); //prints first value of tuple tup1, ea 20 
    println!("{}", (tup1.7).0);
    println!("{}", mac);

    print_number_to(5);





}

fn print_number_to(num: u32){
    for n in 1..num { //non-inclusive 
        println!("{}", n);
    }
}
