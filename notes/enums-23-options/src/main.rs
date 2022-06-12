use std::fs::File;

fn main() {
    let res = File::open("foo");
    //let f   = res.unwrap();
    
    //lef f   = res.expect("error message");
    
    /*if res.is_ok() {
        let f = res.unwrap();
    }*/

    match res {
        Ok(f)   => { /*do stuff*/ },
        Err(e)  => {/*do other stuff */},
    }
}
