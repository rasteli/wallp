use inquire::{error::InquireResult, Select};
use itertools::Itertools;
use std::collections::HashMap;

use crate::services::{add_wallp_dirs, preview_or_apply_wallpapers};
use crate::utils::bootstrap::config;

mod services;
mod structs;
mod utils;

enum CodeCases {
    Set,
    Preview(bool),
}

impl CodeCases {
    fn exec(&self, wallp_dir: String, directories_file: String) {
        use CodeCases::*;

        match self {
            Set => add_wallp_dirs(wallp_dir, directories_file),
            Preview(preview) => {
                let code_case = preview_or_apply_wallpapers(wallp_dir, directories_file, *preview);

                if code_case.is_err() {
                    let action = if *preview { "preview" } else { "apply" };
                    println!("Failed to {action} wallpaper(s)");
                }
            }
        }
    }
}

fn main() -> InquireResult<()> {
    let (wallp_dir, directories_file) = config()?;

    let cases = HashMap::from([
        ("Set wallpapers directories", CodeCases::Set),
        ("Apply wallpapers", CodeCases::Preview(false)),
        ("Preview wallpapers", CodeCases::Preview(true)),
    ]);

    let answer =
        Select::new("What do you want to do?", cases.keys().sorted().collect()).prompt()?;

    if !cases.contains_key(answer) {
        return Err(inquire::InquireError::Custom("Invalid selecition".into()));
    }

    cases[answer].exec(wallp_dir, directories_file);

    Ok(())
}
