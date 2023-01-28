fn main() {

    {
        //replace
        let my_string = String::from("Rust is awesome!");
        println!("after replace: {}", my_string.replace("awesome", "the best"));
    }
    println!("Hello, world!");

    {
        //lines
        let my_string = String::from("Rust\n is\n awesome!");
        println!("{}",my_string);
        
        for line in my_string.lines(){
            print!("[ {} ]", line);
        }

        println!("");
    }

    {
        //split
        let my_string = String::from("Rust is the best!");
        let tokens: Vec<&str> = my_string.split(" ").collect(); //.split --> iterator, .collect converts to vector
        
        println!("index 3: {}", tokens[3]);
    }

    {
        //trim
        let my_string = String::from("     Rust     is    the best    !  ");

        print!("no trim:{} ", my_string);
        println!("/end");

        print!("trim:{} ", my_string.trim());
        println!("/end");

        println!("[conclusion on .trim() \nremoves all white spaces in front,\nall but one at the end,\nand non in the middle.]");
    }
}
