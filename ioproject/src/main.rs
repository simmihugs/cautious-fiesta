use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let search_string = args.get(1).ok_or("Missing search string")?;
    let file_path = args.get(2).ok_or("Missing file")?;
    let content = read_file(&file_path)?;

    println!("searchstring: {search_string:?}");
    println!("filepath: {file_path:?}");
    println!("content: {content:?}");

    Ok(())
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let file = std::fs::read_to_string(&file_path)?;

    Ok(file)
}
