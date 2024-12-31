use serde::{Deserialize, Serialize};
use serde_json;
use tokio::fs;

use std::path;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub origin_dir: PathBuf,
    pub target_dir: PathBuf,
}

pub async fn from_filepath(target_filepath: &PathBuf) -> Result<Config, String> {
    if !target_filepath.is_file() {
        return Err("config -- args path is not a file".to_string());
    }

    let config_pathbuf = match path::absolute(target_filepath) {
        Ok(pb) => pb,
        _ => return Err("config -- could not create an absolute path from args path".to_string()),
    };

    // build json conifg
    let json_as_str = match fs::read_to_string(&config_pathbuf).await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    let mut config: Config = match serde_json::from_str(&json_as_str) {
        Ok(j) => j,
        Err(e) => return Err(e.to_string()),
    };

    let parent_dir = match config_pathbuf.parent() {
        Some(directory) => directory,
        _ => return Err("no parent directory!, crazy!".to_string()),
    };

    config.origin_dir = parent_dir.join(config.origin_dir);
    if !config.origin_dir.is_dir() {
        return Err("config.origin_dir is not a directory.".to_string());
    }
    config.target_dir = parent_dir.join(config.target_dir);
    if !config.target_dir.is_dir() {
        return Err("config.target_dir is not a directory.".to_string());
    }

    Ok(config)
}
