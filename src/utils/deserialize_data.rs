use super::read_file;
use crate::structs::Data;
use std::process;

pub fn deserialize_data(config_file: &String) -> Data {
    let content = match read_file(&config_file[..]) {
        Ok(content) => content,
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", config_file);
            // Exit the program with exit code `1`.
            process::exit(1);
        }
    };

    let data: Data = toml::from_str(&content).unwrap();

    data
}
