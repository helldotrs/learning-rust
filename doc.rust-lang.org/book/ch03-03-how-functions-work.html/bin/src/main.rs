fn main(){
  another_function(5); //snake_case for functions in rust as per convention.
  
  print_labeled_measurement(5, 'h'); //single qoute for char

  fn scope_test(){
    println!("yay! :D");
  }
  scope_test();
 
  
}

fn another_function(x: i31){  //must declare value type for each parameter in a function signature
  println!("Value of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char){
  println!("Measurement: {value}{unit_label}"); // is there any convention for when to use "{x}" vs "{}", x??
}
