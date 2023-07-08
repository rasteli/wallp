use clap::Parser;
use inquire::{error::InquireResult, Select};
use itertools::Itertools;
use std::collections::HashMap;

use crate::services::{
    add_wallp_dirs::add_wallp_dirs, preview_or_apply_wallpapers::preview_or_apply_wallpapers,
};
use crate::utils::bootstrap::config;

mod services;
mod structs;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    monitors: usize,
}

enum CodeCases {
    Set,
    Preview(bool),
}

impl CodeCases {
    fn exec(&self, wallp_dir: String, directories_file: String, args: Args) {
        use CodeCases::*;

        match self {
            Set => add_wallp_dirs(wallp_dir, directories_file),
            Preview(preview) => {
                let code_case = preview_or_apply_wallpapers(
                    wallp_dir,
                    directories_file,
                    *preview,
                    args.monitors,
                );

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

    let args = Args::parse();

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

    cases[answer].exec(wallp_dir, directories_file, args);

    Ok(())
}
