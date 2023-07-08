use crate::utils::{read_file::read_file, spawn_feh::spawn_feh};
use inquire::{
    error::InquireResult, list_option::ListOption, validator::Validation, CustomType, MultiSelect,
};
use regex::Regex;
use std::{
    fs,
    path::{Display, PathBuf},
};

pub fn preview_or_apply_wallpapers(
    wallp_dir: String,
    directories_file: String,
    preview: bool,
    monitor_count: usize,
) -> InquireResult<()> {
    let file_path = format!("{}/{}", wallp_dir, directories_file);

    let dirs = read_file(&file_path[..]).unwrap();
    let mut dirs = dirs.split('\n').collect::<Vec<_>>();

    // Remove EOF character
    dirs.remove(dirs.len() - 1);

    let mut entries: Vec<PathBuf> = vec![];

    for dir in dirs {
        match fs::read_dir(dir) {
            Ok(content) => content.for_each(|e| entries.push(e.unwrap().path())),
            Err(_) => println!("Failed to read path {}", dir),
        }
    }

    entries.sort();

    // Supported image types
    let re = Regex::new(r"^.*\.(jpe?g|png|webp)$").unwrap();

    let entries = entries
        .iter()
        .filter(|e| re.captures(&e.display().to_string()[..]).is_some())
        .map(|e| e.display())
        .collect::<Vec<_>>();

    let validator = move |input: &[ListOption<&Display>]| {
        if input.is_empty() {
            return Ok(Validation::Invalid("Select at least one image".into()));
        }

        if input.len() > monitor_count {
            return Ok(Validation::Invalid(
                format!("Select at maximum {} image(s)", monitor_count).into(),
            ));
        }

        Ok(Validation::Valid)
    };

    let images = MultiSelect::new("Select wallpapers", entries)
        .with_page_size(20)
        .with_validator(validator)
        .with_help_message(format!("Select up to {} image(s)", monitor_count).as_str())
        .prompt()?;

    let mut images: Vec<String> = images.iter().map(|img| img.to_string()).collect();

    if images.len() > 1 {
        for img in images.clone().iter().take(monitor_count) {
            let monitor_index: usize =
                CustomType::new(format!("Apply image {} to monitor", img).as_str())
                    .with_placeholder("0")
                    .with_error_message("Please type a valid number")
                    .with_help_message(format!("Type a number (0-{})", monitor_count - 1).as_str())
                    .prompt()?;

            images[monitor_index] = img.to_string();
        }
    }

    spawn_feh(images, preview);

    Ok(())
}
