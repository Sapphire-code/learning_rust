// cargo.toml -> colored = "2.0.0"
use colored::*;
// cargo.toml -> rand = "0.5.5"
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        let mut secret_number = rand::thread_rng().gen_range(1, 101);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("The secret number is {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Number too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
