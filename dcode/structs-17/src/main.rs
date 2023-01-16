struct Color {
    red: u8, // u8  -> 0-255
    green: u8,
    blue: u8
}

fn main(){
    let mut bg = Color { red: 225, green: 70, blue: 15};

    println!("bg: {}, {}, {}", bg.red, bg.green, bg.blue)

    bg.blue = 45;

    println!("bg: {}, {}, {}", bg.red, bg.green, bg.blue)
}
