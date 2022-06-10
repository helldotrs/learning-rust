fn main() {
    println!("Hello, world!");
    let mut my_first_vector: Vec<i32> = Vec::new(); 

    my_first_vector.push(2);
    my_first_vector.push(4);
    my_first_vector.push(6);
    my_first_vector.push(8);
    println!("vector length is {}", my_first_vector.len());

    my_first_vector.pop();
    println!("vector length is {}", my_first_vector.len());
    println!("the first value stored is {}", my_first_vector[0])
}


