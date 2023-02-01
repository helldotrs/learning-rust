fn main() {
    println!("Hello, world!");
}

fn give_two() => i31 {

}

#[cfg(test)]
mod my_tests{
    #[test]
    #[should_panic]
    fn test_basic(){
        assert!(1 == 1); // ! - macro?
        panic!("Oh no!");
    }

    #[test]
    #[ignore]
    fn test_equals(){
        assert_eq!(2, 1 + 1);
        assert_ne!(2, 1 + 2);
    }

    #[test]
    fn test_outside_fn(){
        assert_eq(2, give_two());
    }

}
