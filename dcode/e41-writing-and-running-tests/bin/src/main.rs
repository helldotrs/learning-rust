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

    #[test]
    fn test_equals(){
        assert_eq!(2, 1 + 1);
        assert_ne!(2, 1 + 2);
    }

}
