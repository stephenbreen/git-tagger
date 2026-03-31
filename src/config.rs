use serde::Deserialize;
use ratatui::style::Color;
use std::fs;
use std::path::PathBuf;
use color_eyre::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub highlights: Vec<Highlight>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Highlight {
    pub pattern: String,
    pub color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            highlights: vec![
                Highlight {
                    pattern: "-staging".to_string(),
                    color: "green".to_string(),
                },
                Highlight {
                    pattern: "-prod".to_string(),
                    color: "red".to_string(),
                },
                Highlight {
                    pattern: "-production".to_string(),
                    color: "red".to_string(),
                },
            ],
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = PathBuf::from("git-tagger.toml");
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
}

pub fn string_to_color(s: &str) -> Color {
    match s.to_lowercase().as_str() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "black" => Color::Black,
        _ => Color::Reset,
    }
}
