use std::{fs::File, io::ErrorKind};

fn main() {
    let file_path = String::from("hello.txt");

    let _file = File::open(&file_path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&file_path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
