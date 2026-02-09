use std::io;

fn main() {
    print!("Enter text: ");

    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("failed to read line");

    println!("You entered: {}", buf);
}
