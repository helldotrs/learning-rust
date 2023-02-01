use std::process::Command;


fn main() {
    let mut my_cmd = Command::new("python");
    my_cmd.arg("script.py");

    match my_cmd.output() {
        Ok(a)   => {
            unsafe{
                println!("output: {}", String::from_utf8_unchecked(a.stdout));
            }
        }

        Err(a)  => {
            println!("error: {}", a);
        }

    }

    println!("Hello, world!");
}
