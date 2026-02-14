use std::fs::File;

fn open_file(file_name: &str) -> Result<File, std::io::Error> {
    let res = File::open(file_name)?;

    Ok(res)
}

fn main() {
    let file_result = open_file("hello.txt");

    match file_result {
        Err(e) => println!("{e:?}"),
        Ok(_) => println!("it worked!"),
    }
}
