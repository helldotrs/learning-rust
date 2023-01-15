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

    for a in animals.iter() {//local a for animals  //using iter() method prevents ownership of the vector to being moved to the for-loop
        println!("{} is an animal", a);
    }





}
