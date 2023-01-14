fn main() {
    //single line comment
    /*
    multi line comment
    */

    let mut a: i32  = 45;    // i32
                            // u64 for "unsigned 64 bit integer" does not support negative numbers
    let _b          = 4.5;  // f32 - f for float
    let _c: bool    = true; // boolean // _ for unused variable
    let test_value  = 40;
    let tv          = &test_value; //fixme - come back to this.


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


}
