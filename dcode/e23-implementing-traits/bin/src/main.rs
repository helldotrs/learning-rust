struct Person {
    name: String,
    age: u8
}

impl ToString for Person{
    fn to_string(&self) -> String {
        format!("My name is {} and I am {}.", self.name, self.age)
    }
}

fn main() {
    let bekah = Person { name: String::from("Rebekah"), age: 123};

    println!("{}", bekah.to_string());
}
