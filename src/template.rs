//! Template resolution and copying.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use serde::Deserialize;

pub(crate) static TEMPLEX_DIRECTORY: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut dir = directories::UserDirs::new().unwrap().home_dir().to_owned();
    dir.push(".templex/");
    if let Ok(exists) = fs::exists(&dir) {
        if !exists {
            if let Err(e) = fs::create_dir_all(&dir) {
                eprintln!("Failed to create .templex directory: {}", e);
            } else {
                println!("Successfully created .templex directory at {dir:?}");
            }
        }
    }
    dir
});

#[derive(Deserialize)]
pub struct TemplateConfiguration {
    pub folders_to_create: Vec<PathBuf>,
    pub files_to_copy: Vec<PathBuf>,
}

/// Reads a template given a [`PathBuf`] pointing to
/// the `template.templex.toml` file.
pub fn read_template(config_file: impl AsRef<Path>) -> Result<TemplateConfiguration, Box<dyn std::error::Error + 'static>> {
    let file = config_file.as_ref();
    let content = fs::read_to_string(file)?;
    let tc: TemplateConfiguration = toml::from_str(content.as_str())?;
    Ok(tc)
}

pub fn get_templates() -> Vec<String> {
    let mut vec = vec![];
    let mut x = TEMPLEX_DIRECTORY.clone();
    x.push("templates/");
    if !x.exists() {
        let _ = fs::create_dir_all(&x);
        return vec;
    }
    if let Ok(read_dir) = x.read_dir() {
        for dir in read_dir {
            if let Ok(dir) = dir {
                vec.push(dir.file_name().to_string_lossy().to_string());
            }
        }
    }
    vec
}

pub fn resolve_template(name: String) -> Option<PathBuf> {
    let mut x = TEMPLEX_DIRECTORY.clone();
    x.push("templates/");
    if !x.exists() {
        let _ = fs::create_dir_all(&x);
        return None;
    }
    x.push(name + "/");
    if x.exists() {
        Some(x)
    } else {
        None
    }
}