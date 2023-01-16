fn main() {
    let mut a = 10;
    //let ar = &a;

    {
        let ar2 = &mut a;

        *ar2 += 1;
    }

    println!("a is {}", a)
}
