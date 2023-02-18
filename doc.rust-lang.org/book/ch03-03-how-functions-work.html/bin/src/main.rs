/* only mention of multi-lines in "Comments" is:
For comments that extend beyond a single line, you’ll need to include // on each line, like this:

// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.


-why?
src: https://doc.rust-lang.org/book/ch03-04-comments.html
*/

/*
-expressions, {}, no semicolon at the end, function(){5} returns 5.
  macros (ea. println!)??
-statement ends with ;
-..gone by instinct with ; up until this point..
*/

fn main(){
  another_function(5); //snake_case for functions in rust as per convention.
  
  print_labeled_measurement(5, 'h'); //single qoute for char

  fn scope_test(){
    println!("yay! :D");
  }
  scope_test();
  
  fn get_five() -> i32 {
  5 // without return and ; only if in the end of the function/expression?
  }
  let five = get_five();
  println!("{five}");
  
  // controll flow:
  let number = 3;

  if number < 5 {
      println!("condition was true");  
       // Blocks of code associated with the conditions in if expressions are sometimes called arms
  } else {
      println!("condition was false");
  }
  
  
  
   //Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.
   //instead of: if number {}
   //use: if number != 0 {}
 
  
}

fn another_function(x: i31){  //must declare value type for each parameter in a function signature
  println!("Value of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char){
  println!("Measurement: {value}{unit_label}"); // is there any convention for when to use "{x}" vs "{}", x??
}




