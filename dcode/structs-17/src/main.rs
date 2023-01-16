struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main(){
    let bg = Color { red: 225, green: 70, blue: 15};

    println!("bg: {}, {}, {}", bg.red, bg.green, bg.blue)
}
