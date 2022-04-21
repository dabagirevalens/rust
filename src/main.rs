use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome To Guessing Game.!!");
    println!("============================");
    println!("Guess the number between 1 to 100");

    loop {
        println!("\n\nEnter your guess:");
        let secret_num = rand::thread_rng().gen_range(1..101);
        // println!("The secret number is: {}", secret_num);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations You Have Won..!!!");
                break;
            }
        }
    }
}
