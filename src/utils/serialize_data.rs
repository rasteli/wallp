use std::{fs, io};
use toml;

use crate::structs::Data;

pub fn serialize_data(config_file: &str) -> Result<(), io::Error> {
    let data = Data::default();

    let toml = toml::to_string(&data).unwrap();

    let toml = 
        format!("# Colors are ANSI values.\n# See https://www.ditig.com/256-colors-cheat-sheet for more info.\n\n{}", toml);

    fs::write(config_file, toml)?;

    Ok(())
}
