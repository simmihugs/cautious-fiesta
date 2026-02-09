use rand::RngExt;
use std::{cmp::Ordering, io};

fn main() {
    let mut buf = String::new();
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Enter your guess between 0 and 100: ");

        io::stdin()
            .read_line(&mut buf)
            .expect("failed to read line");

        let guess = match buf.trim().parse() {
            Ok(i) => i,
            Err(_) => 42,
        };
        buf.clear();

        if guess == 42 {
            println!("Aaaah, the answer to the universe, all and everything! Not the secret number though!");
            continue;
        }

        match secret_number.cmp(&guess) {
            Ordering::Greater => {
                println!("You guess `{guess}` is smaller then the secret number {secret_number}!")
            }
            Ordering::Less => {
                println!("You guess `{guess}` is greater then the secret number {secret_number}!")
            }
            Ordering::Equal => {
                println!("You guessed the secret number!");
                break;
            }
        };
    }
}
