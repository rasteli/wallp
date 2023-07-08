use colored::Colorize;
use inquire::{required, Text};
use std::io::prelude::*;
use std::{fs, path::Path};

pub fn add_wallp_dirs(wallp_dir: String, directories_file: String) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}/{}", wallp_dir, directories_file))
        .unwrap();

    let answer = Text::new("Wallpaper directories:")
        .with_help_message("Type a space to specify multiple directories")
        .with_validator(required!("Specify at least one directory"))
        .prompt()
        .unwrap();

    let dirs = answer.split(' ');

    for dir in dirs {
        let path_exists = Path::new(dir).exists();

        if !path_exists {
            println!(
                "{} {}",
                "No such directory".red().bold(),
                dir.red().bold().italic()
            );

            continue;
        }

        match writeln!(file, "{dir}") {
            Ok(_) => {
                println!(
                    "{} {} {}",
                    "Directory".blue().bold(),
                    dir.blue().bold().italic(),
                    "successfuly set".blue().bold()
                );
            }
            Err(_) => {
                println!(
                    "{} {}",
                    "Failed to write to file".red().bold(),
                    dir.red().bold()
                );
            }
        };
    }
}
