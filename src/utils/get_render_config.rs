use home::home_dir;
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};
use std::{io, path::Path};

use super::{deserialize_data, serialize_data};

fn leak(str: String) -> &'static str {
    Box::leak(str.into_boxed_str())
}

pub fn get_render_config() -> Result<RenderConfig, io::Error> {
    let home_dir = home_dir().unwrap();
    let home_dir = home_dir.display();

    let config_file = format!("{}/.config/wallp.toml", home_dir);

    if !Path::new(&config_file[..]).exists() {
        serialize_data(&config_file)?
    }

    let data = deserialize_data(&config_file);

    let mut render_config = RenderConfig::default();

    let prompt_prefix = leak(data.prompt.prefix);
    let highlighted_option_prefix = leak(data.highlighted_option.prefix);
    let selected_checkbox = leak(data.checkbox.selected);
    let unselected_checkbox = leak(data.checkbox.unselected);
    let scroll_up_prefix = leak(data.scroll.up_prefix);
    let scroll_down_prefix = leak(data.scroll.down_prefix);
    let error_prefix = leak(data.error.prefix);

    render_config.prompt_prefix =
        Styled::new(prompt_prefix).with_fg(Color::AnsiValue(data.prompt.color));

    render_config.scroll_up_prefix =
        Styled::new(scroll_up_prefix).with_fg(Color::AnsiValue(data.scroll.up_color));

    render_config.scroll_down_prefix =
        Styled::new(scroll_down_prefix).with_fg(Color::AnsiValue(data.scroll.down_color));

    render_config.unselected_checkbox = Styled::new(unselected_checkbox);

    render_config.selected_checkbox =
        Styled::new(selected_checkbox).with_fg(Color::AnsiValue(data.checkbox.selected_color));

    render_config.highlighted_option_prefix = Styled::new(highlighted_option_prefix)
        .with_fg(Color::AnsiValue(data.highlighted_option.color));

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new(error_prefix).with_fg(Color::AnsiValue(data.error.color)));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::AnsiValue(data.answer.color));

    render_config.help_message =
        StyleSheet::new().with_fg(Color::AnsiValue(data.help_message.color));

    Ok(render_config)
}
