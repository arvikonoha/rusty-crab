fn main()
{
  let name = String::from("Hello world");
  let n_name = take_and_give(name); // yeh karna padtha hai,
  // ownership maintain karne ke lie
  println!("{}",n_name);
  // string stores data in a heap, and name has the following details
  // in the stack pointer to heap, length and capacity
  // let cunt = name;
  // println!("{}",name); // name's ownership is moved to cunt, Drop/Move trait

  // let x = 16;
  // let y = x;

  // println!("{} {}",x,y) // but there is no mocing ownership in the case
  // of scalar values, Copy trait
}

fn take_and_give(name: String) -> String{
  name
} 