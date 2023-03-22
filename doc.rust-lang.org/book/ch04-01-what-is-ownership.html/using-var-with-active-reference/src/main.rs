  GNU nano 5.4                                                  src/main.rs *                                                          
fn main() {
        let sv = String::from("Rust!");
        println!("{}", sv);
        {
                let sr = &sv;
                println!("{}", sr); //should compile
                println!("{}", sv); //compiles? it does, conslussion: I can call a String var while a reference is still in scope?
                                        //or is the reference dropped once it is not used any more? lets test.
                println!("{}", sr); //still compiles!

        }
}


