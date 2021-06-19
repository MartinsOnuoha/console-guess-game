#![allow(unused_imports, unused_variables)]
use std::io;

fn main() {
    println!("🎲 GUESS THE NUMBER!!");
    println!("Enter your Guess: ");

    let mut guess: String = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_n) => {
            println!("You guessed: {}", guess);
        }
        Err(error) => println!("Failed to read input, {}", error)
    }
}
