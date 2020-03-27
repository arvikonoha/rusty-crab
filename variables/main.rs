fn main()
{
  // let a = "Mutable"; // immutable by default
  // a = "This is not possible";

  // let mut a = "Immutable";
  // a = "Lund";

  let a = "Hello world";
  println!("Hello world, this variable is {}!",a);
  
  // shadowing
  let a = 69;
  println!("Hello world, this variable is {}!",a);
}