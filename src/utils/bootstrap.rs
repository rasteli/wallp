use super::get_render_config::get_render_config;
use home::home_dir;
use inquire::ui::RenderConfig;
use std::{fs, io::Error, path::Path};

pub fn config() -> Result<(String, String), Error> {
    let render_config = match get_render_config() {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Failed to load render config");
            RenderConfig::default()
        }
    };

    inquire::set_global_render_config(render_config);

    let home_dir = home_dir().unwrap();

    let wallp_dir = format!("{}/.local/share/wallp", home_dir.display());
    let directories_file = "directories".to_string();

    if !Path::new(&wallp_dir[..]).exists() {
        fs::create_dir_all(&wallp_dir)?
    }

    Ok((wallp_dir, directories_file))
}
