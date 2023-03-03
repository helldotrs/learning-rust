fn main() {
    // Create a string and bind it to the variable `s1`.
    let mut s1 = String::from("hello");

    // Pass ownership of the string to the `take_ownership` function.
    take_ownership(s1);

    // Attempt to use `s1` again; this will result in a compiler error since ownership has been transferred.
    // Uncomment the following line to see the error:
    // println!("s1: {}", s1);

    // Create another string and bind it to the variable `s2`.
    let mut s2 = String::from("world");

    // Pass a mutable reference to the string to the `borrow` function.
    borrow(&mut s2);

    // Use `s2` again; since we only borrowed it, it is still valid.
    println!("s2: {}", s2);
}

// `take_ownership` takes ownership of a string and drops it at the end of the function.
fn take_ownership(s: String) {
    println!("Inside take_ownership: {}", s);
}

// `borrow` takes a mutable reference to a string, but doesn't take ownership, so the string is still valid when the function ends.
fn borrow(s: &mut String) {
    s.push('!');
    println!("Inside borrow: {}", s);
}
