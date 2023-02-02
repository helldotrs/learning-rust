extern crate serde_json;

use serde_json::Value as JsonValue;

fn main() {
    let json_str = r#"
        {
            "name": "Domenic":,
            "age": 65,
            "is_male": true
        }
    "#; // Rust raw-string

    let res = serde_json::from_str(json_str);


    println!("Hello, world!");
}
