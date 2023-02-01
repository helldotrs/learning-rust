fn main() {
    println!("Hello, world!");
}

mod my_tests{
    #[test]
    #[should_panic]
    fn test_basic(){
        assert!(1 == 1); // ! - macro?
        panic!("Oh no!");
    }
}
