use crate::lib::utils::read_file;
use std::error::Error;

pub struct Config {
    query: String,
    content: String,
}

impl Config {
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub fn parse_config(args: &[String]) -> Result<Config, Box<dyn Error>> {
    let search_string = args.get(1).ok_or("Missing search string")?;
    let file_path = args.get(2).ok_or("Missing file")?;
    let content = read_file(&file_path)?;

    Ok(Config {
        query: search_string.to_string(),
        content: content,
    })
}
