use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::util;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub api_key: String,
    pub gist_id: String,
}

impl Settings {
    pub fn load() -> Self {
        let content = fs::read_to_string(settings_path()).unwrap();
        serde_yaml::from_str(&content).unwrap()
    }
}

fn settings_path() -> PathBuf {
    util::home_dir().join(".config/hours.yaml")
}
