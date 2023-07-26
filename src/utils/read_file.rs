use std::error::Error;
use std::fs;

pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(file_path)?.parse()?)
}
