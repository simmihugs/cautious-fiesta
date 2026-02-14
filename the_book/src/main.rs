use std::{
    fs::File,
    io::{self, Read},
};

fn read_user_name_from_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main() {
    let file_path = String::from("hello.txt");

    let _res = read_user_name_from_file(&file_path).unwrap_or_else(|error| {
        println!("{error:?}");
        String::new()
    });
}
