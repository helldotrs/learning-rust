fn main() {

    {
        let my_string = String::from("Rust is awesome!");
        println!("after replace: {}", my_string.replace("awesome", "the best"));
    }
    println!("Hello, world!");

    {
        let my_string = String::from("Rust\n is\n awesome!");
        println!("{}",my_string);
        
        for line in my_string.lines(){
            print!("[ {} ]", line);
        }
        
        println!("");
    }

    {
        let my_string = String::from("Rust is the best!");
        let tokens: Vec<&str> = my_string.split(" ").collect(); //.split --> iterator, .collect converts to vector
        
        println!("index 3: {}", tokens[3]);
    }
}
