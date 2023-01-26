fn main() {
    // "a vector is basically this beefed up array on steriods. is a style of structure where you can store a sequence of elements inside"
    
    //let mut my_vector: Vec<i32> = Vec::new(); // easier way:
    let mut my_vector   = vec![1,2,3,4];

    println!("{}", my_vector[0]); //also adds 1 
    
    println!("");

    my_vector.push(5);
    my_vector.remove(1); //removed second  element, ea, 2

    for element in my_vector.iter(){
        println!("{}", element);
    }
}
