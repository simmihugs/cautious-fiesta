use std::{
    fs::{self, File},
    io::{self, Read},
};

fn read_user_name_from_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn read_user_name_from_file_easy(file_path: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let file_path = String::from("hello.txt");

    let _res = read_user_name_from_file(&file_path).unwrap_or_else(|error| {
        println!("{error:?}");
        String::new()
    });
    let _res = read_user_name_from_file_easy(&file_path).unwrap_or_else(|error| {
        println!("{error:?}");
        String::new()
    });

    match last_char_of_first_line("hello\nworld!") {
        Some(c) => println!("{c}"),
        None => println!("no char found"),
    }
}
