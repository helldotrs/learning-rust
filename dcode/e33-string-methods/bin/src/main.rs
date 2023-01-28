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
    }
}
