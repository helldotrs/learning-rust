extern crate reqwest;

fn main(){
    let response_text = reqwest::get("hellmak.com")
    .expect("could not make request").text().expect("could not read the response text");

    println!("response: {}", response_text);
}

/*extern crate reqwest;

fn main() {
    match reqwest::get("http://google.com"){
        Ok(mut reponse) => {
            //check if 200 OK
            if reponse.status() == reqwest::StatusCode::Ok{
                match response.text(){
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println("could not print response text")
                }
            }
            else
            {
                println!("response was not 200 OK")
            }
        }
        Err(_) => println!("could not make the request");
    }

    println!("Hello, world!");
}*/
/*
[package]
name = "bin"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
reqwest = "0.11.14"
*/
