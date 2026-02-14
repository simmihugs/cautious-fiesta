use std::{fs::File, io::ErrorKind};

fn open_file(file_name: &str) -> Result<File, std::io::Error> {
    let res = File::open(file_name)?;

    Ok(res)
}

fn main() {
    let file_path = String::from("hello.txt");
    let file_result = open_file(&file_path);

    match file_result {
        Ok(_) => println!("it worked!"),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(&file_path) {
                Ok(_) => println!("it worked to create file!"),
                Err(e) => panic!("Problme creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {e:?}");
            }
        },
    };
}
