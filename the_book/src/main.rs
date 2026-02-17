use std::env;

fn build(mut args: impl Iterator<Item = String>) -> Result<(), &'static str> {
    args.next();
    let a = match args.next() {
        Some(arg) => arg,
        None => return Err("No search string"),
    };
    let b = match args.next() {
        Some(arg) => arg,
        None => return Err("No file"),
    };

    println!("searchstring: {a}\tfile: {b}");

    Ok(())
}

fn main() -> Result<(), &'static str> {
    build(env::args())?;

    Ok(())
}
