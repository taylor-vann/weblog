use serde::{Deserialize, Serialize};
use serde_json;
use tokio::fs;

use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub directory: PathBuf,
    pub host_and_port: String,
    pub content_encodings: Option<Vec<String>>,
    pub filepath_404: Option<PathBuf>,
    pub pages: Vec<(String, PathBuf)>,
}

pub async fn from_filepath(target_path: &PathBuf) -> Result<Config, String> {
    // build json conifg
    let json_as_str = match fs::read_to_string(&target_path).await {
        Ok(r) => r,
        Err(_e) => return Err("failed to read config from json file".to_string()),
    };
    let mut config: Config = match serde_json::from_str(&json_as_str) {
        Ok(j) => j,
        Err(_e) => return Err("failed to parse config from json string".to_string()),
    };

    let parent_dir = match target_path.parent() {
        Some(directory) => directory,
        _ => return Err("no parent directory!, crazy!".to_string()),
    };

    config.directory = match fs::canonicalize(parent_dir.join(config.directory)).await {
        Ok(dir) => dir,
        Err(_e) => return Err("could not create absolute path from config-directory".to_string()),
    };

    Ok(config)
}
