use std::io::{self, stdin};
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number");
    
    let secret_number=rand::thread_rng().gen_range(1..100);
    println!("Please input ur number");
    let mut guess=String::new();
  
    let stdin=io::stdin();
    stdin.read_line(&mut guess);
    //    .expect("Failedc to read the line");
    let guess:u32=guess.trim().parse().unwrap();
    println!("You guessed:{}",guess);
    println!("Number u had to guess {}",secret_number);
    match guess.cmp(&secret_number){
        Ordering::Equal=>println!("You win"),
        Ordering::Greater=>println!("Too big"),
        Ordering::Less=>println!("Too small"),
    }
}
