fn main() {
    println!("Hello, world!");
}

mod my_tests{
    #[test]
    fn test_basic(){
        assert!(1 == 1); // ! means macro?
    }
}
