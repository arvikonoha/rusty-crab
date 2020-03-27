use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let secret_num = rand::thread_rng().gen_range(1,101);
    
    loop{

        let mut number = String::new();

        println!("Pick a number");
        io::stdin().read_line(&mut number).expect("Something went wrong");
        println!("The number you picked is {}",number);
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(err) => {println!("{}",err );continue;}
        };
        match number.cmp(&secret_num) {
            Ordering::Less => println!("Less than the actual number"),
            Ordering::Greater => println!("Greater than the actual number"),
            Ordering::Equal => {println!("Bingo!");break;}
        };
    }
}
