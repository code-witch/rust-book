use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {

    let rand_number = rand::thread_rng().gen_range(1..=100);
    println!("Random number: {rand_number}");
    println!("Guess a Number from 1-100: ");
    loop {
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

    }

}
