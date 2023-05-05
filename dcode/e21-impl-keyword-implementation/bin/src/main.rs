struct Rectangle {
    width:  u32,
    height: u32
}

impl Rectangle {
    fn print_description(&self){
        println!("Rectangle: {} x {}", self.width, self.height)
    }

    fn is_square(&self) -> bool { //returns boolean
        self.width == self.height //no ; hence a return value
    }
}

fn main() {
    let my_rect     = Rectangle{width: 10, height: 5};
    let my_square   = Rectangle{width: 10, height: 10};

    my_rect.print_description();
    println!("my_rect is a square: {}", my_rect.is_square());
    println!("my_square is a square: {}", my_square.is_square());
}
