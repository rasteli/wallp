use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Prompt {
    pub prefix: String,
    pub color: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Error {
    pub prefix: String,
    pub color: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Checkbox {
    pub selected: String,
    pub unselected: String,
    pub selected_color: u8,
    pub unselected_color: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Scroll {
    pub up_prefix: String,
    pub down_prefix: String,
    pub up_color: u8,
    pub down_color: u8,
}

#[derive(Deserialize, Serialize)]
pub struct HighlightedOption {
    pub prefix: String,
    pub color: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Answer {
    pub color: u8,
}

#[derive(Deserialize, Serialize)]
pub struct HelpMessage {
    pub color: u8,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub prompt: Prompt,
    pub error: Error,
    pub checkbox: Checkbox,
    pub scroll: Scroll,
    pub highlighted_option: HighlightedOption,
    pub answer: Answer,
    pub help_message: HelpMessage,
}

impl Data {
    pub fn default() -> Self {
        let prompt = Prompt {
            prefix: "❯".to_string(),
            color: 9,
        };

        let error = Error {
            prefix: "✘".to_string(),
            color: 9,
        };

        let checkbox = Checkbox {
            selected: "◉".to_string(),
            unselected: "◯".to_string(),
            selected_color: 40,
            unselected_color: 8,
        };

        let scroll = Scroll {
            up_prefix: "▲".to_string(),
            down_prefix: "▼".to_string(),
            up_color: 8,
            down_color: 8,
        };

        let highlighted_option = HighlightedOption {
            prefix: "❯".to_string(),
            color: 11,
        };

        let answer = Answer { color: 11 };

        let help_message = HelpMessage { color: 8 };

        Data {
            prompt,
            error,
            checkbox,
            scroll,
            highlighted_option,
            answer,
            help_message,
        }
    }
}
