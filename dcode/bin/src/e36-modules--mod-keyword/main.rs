mod dcode{
    fn chicken(){
        println!("ka-kaaw!");
    }
    pub fn print_msg(){
        println!("Hello Kitty!");
        chicken();
    }
    pub mod nested_module{
        pub fn print_msg(){
            println!("Ohai!");
        }
    }
}

fn main() {
    dcode::print_msg();
    dcode::nested_module::print_msg();

    println!("Hello, world!");
}
