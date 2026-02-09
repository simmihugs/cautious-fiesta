use rand::RngExt;
use std::io;

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

        if guess == secret_number {
            println!("You guessed the secret number!");
            break;
        } else {
            println!("You guess `{guess}` does not match the secret number!");
        }
    }
}
