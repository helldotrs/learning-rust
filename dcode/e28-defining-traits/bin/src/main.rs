//a trait like a certain set of rules or requirements that an object or a struct have to have to be named a trait(?)

struct Person{
    name: String,
    age: u8
}

trait HasVoiceBox {
    // Speak
    fn speak(&self);
    // Check if can speak
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person{
    fn speak(&self){
       println!("Hello my name is {}", self.name); 
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            true
        } else {
            false
        }
    }
}

fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 41
    };
    
    println!("{} can speak: {}", person.name, person.can_speak());
}
