use ioproject::lib::config::parse_config;
use ioproject::lib::search::search;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let query = parse_config(&args)?;
    let res = search(&query.query(), &query.content());
    res.iter().for_each(|x| {
        println!("{}", x);
    });
    Ok(())
}
