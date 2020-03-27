fn main()
{
  let number = 5;
  let conclusion = if number > 5 {
    "Rust is terrible"
  } else {
    "Rust is cool"
  };
  println!("{}",conclusion);
  let mut i = 0;
  let suminem = loop {
    i+=10;
    if i>50 {
      break i - 20;
    }
  };
  println!("{}",suminem);

  while i < 100 {
    println!("Bachchodi");
    i+=10;
  }

  for num in 1..4 {
    println!("{}",num)
  }
}