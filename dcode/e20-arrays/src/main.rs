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

    do_print_color(&bg); //if we do not do this by reference the value of bg will be passed to the function hence we wont be able to run this twice

    let numbers = [1, 2, 5]; //array
    let numbers_more: [i32; 3]= [6,7,8];

    println!("{}", numbers[0]); 

    for i in numbers.iter() { //interesting.
        println!("{}", i);
    }

    println!("-");
    
    for n in 0..numbers.len() {
        println!("{}", numbers[n]);
    }

    println!(" ");
    for n in 0..numbers_more.len(){
        print!("{}, ",n );
    }
    println!(" ");
    println!("{}", numbers_more[0]);

    let array_with_default_of_two_repeated_ten_times = [2; 10];

    println!("");
    for n in 0..array_with_default_of_two_repeated_ten_times.len(){
        print!("{}, ", array_with_default_of_two_repeated_ten_times[n]);
    }
    println!("");



    




}

fn do_print_color(c: &Color){
    println!("{}, {}, {}", c.red, c.green, c.blue);
}
