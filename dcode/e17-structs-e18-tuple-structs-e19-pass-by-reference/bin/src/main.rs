struct Color { //struct
    red: u8, // u8  -> 0-255
    green: u8,
    blue: u8
}

struct Tsc ( //touple struct //TSC -ToupleStructColor
    u8, u8, u8
);

fn main(){
    let mut bg = Color { red: 225, green: 70, blue: 15};

    println!("bg: {}, {}, {}", bg.red, bg.green, bg.blue);

    bg.blue = 45;

    println!("bg: {}, {}, {}", bg.red, bg.green, bg.blue);

    let mut red = Tsc(255, 0 ,0);

    red.2 = 45;

    println!("red: {}, {}, {}", red.0, red.1, red.2);

    print_color(&bg); //if we do not do this by reference the value of bg will be passed to the function hence we wont be able to run this twice




}

fn print_color(c: &Color){
    println!("{}, {}, {}", c.red, c.green, c.blue);
}
