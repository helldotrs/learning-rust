extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person{
    name:   String,
    age:    u8,
    is_male: bool
}

fn main() {
    let json_str = r#"
        {
            "name": "Domenic", 
            "age": 65,
            "is_male": true
        }
    "#; // Rust raw-string

    let res = serde_json::from_str(json_str);

    if res.is_ok(){
        /*let p: JsonValue = res.unwrap();
        println!("The name is {}", p["name"]);*/
        let p: Person = res.unwrap();
        println!("{}", p.name)
    } else {
        println!("could not parse JSON");
    }


    println!("Hello, world!");
}
