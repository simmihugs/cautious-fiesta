pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let file = std::fs::read_to_string(&file_path)?;

    Ok(file)
}
