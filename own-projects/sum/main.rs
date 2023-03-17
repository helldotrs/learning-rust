use std::io;

fn main() {
  println!("input a number");
  
  let mut num1 = String::new();
  io::stdin().read_line(&mut num1).expect("failed to read line");
    
  let num1: f32 = num.trim().parse().expect("please enter valid number"); //should make into a function
    //f32 includes negative, u32 does not
    
  println!("enter another number");
    
  let mut num2 = String::new();
  io::stdin().read_line(&mut num2).expect("failed to read line");
  
  let num2: f32 = num.trim().parse().expect("please enter valid number");
  //shaddowing?

  let sum = num1 + num2;
  
  println!("sum of {} and  {} is {}", num1, num2, sum);
    
    


}
