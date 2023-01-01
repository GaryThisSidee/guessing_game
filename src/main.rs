use std::io::{self, stdin};
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess the number");
    
    let secret_number=rand::thread_rng().gen_range(1..100);
    
    
    loop{
        println!("Please input ur number");
        let mut guess=String::new();
        let stdin=io::stdin();
        stdin.read_line(&mut guess);
        let  guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please try again.");
                continue;
            },
        };
        match guess.cmp(&secret_number){
            Ordering::Equal=>
            {
                println!("{}","You win".green());
                break;
            },
            Ordering::Greater=>println!("{}","Too big".red()),
            Ordering::Less=>println!("{}","Too small".red()),
        }
    }
}
