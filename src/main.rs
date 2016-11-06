extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

//rust vars are immutable by default
// references are immutable by default as well
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess) // references are immutable
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
           
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
