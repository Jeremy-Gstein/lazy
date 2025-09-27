use std::fs;
use std::path::Path;

#[derive(Default, Debug)]
pub struct LazyConfig {
    pub name: String,
    pub image_name: String,
    pub image_tag: String,
    pub image: String,
    pub dockerfile: String,
}

impl LazyConfig {
    pub fn load_default() -> std::io::Result<Self> {
        // Look for .lazy file in cwd or $HOME/.config/
        if Path::new(".lazy").exists() {
            Self::load_from_file(".lazy")
        } else if let Some(home) = dirs::home_dir() {
            let config_path = home.join(".config/.lazy");
            if config_path.exists() {
                Self::load_from_file(config_path.to_str().unwrap())
            } else {
                // Return default config if no file found
                Ok(Self::default())
            }
        } else {
            Ok(Self::default())
        }
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut config = Self::default();

        for line in content.lines() {
            if let Some((key, val)) = parse_key_val(line) {
                match key.as_str() {
                    "NAME" => config.name = val,
                    "IMAGE_NAME" => config.image_name = val,
                    "IMAGE_TAG" => config.image_tag = val,
                    "DOCKERFILE" => config.dockerfile = val,
                    _ => {}
                }
            }
        }

        config.image = format!("{}:{}", config.image_name, config.image_tag);
        Ok(config)
    }
}

// Simple helper to parse a line like `KEY="VALUE"`
fn parse_key_val(line: &str) -> Option<(String, String)> {
    // Strip comments and empty lines
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }
    let key = parts[0].trim().to_string();
    let mut val = parts[1].trim().to_string();
    // Remove quotes if present
    if val.starts_with('"') && val.ends_with('"') {
        val = val[1..val.len() -1].to_string();
    }
    Some((key, val))
}

