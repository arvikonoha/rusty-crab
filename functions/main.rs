fn main()
{
  println!("Hello world");

  another_function();
  param_function(8,6.4);

  println!("{}",exp_function(12))
}

fn another_function()
{
  println!("Love yourz");
}

fn param_function(val: u8,val1: f32)
{
  println!("The value of parameter {}",val);
  println!("The value of parameter {}",val1);
}

fn exp_function(val: i16) -> i16{
  val + 1 // this better be an expression cocksucker
}