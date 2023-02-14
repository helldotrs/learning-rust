fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup; // destructing tuple

    println!("The value of y is: {y}");

    println!("first element is {}", tup.0);
    
    //rust tuple: fixed length, can contain different data types ()
    //rust array: fixed length, must contain same data type []
        //"An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. "
        //arraw[index]
    //rust vector: flexible length, 
}
