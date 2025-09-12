use serde::{Deserialize, Serialize};
use serde_json;
use tokio::fs;

use std::env;
use std::path;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BuilderConfig {
    pub output_dir: PathBuf,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileServerConfig {
    pub host_and_port: String,
    pub directory: PathBuf,
    pub content_encodings: Option<Vec<String>>,
    pub filepath_404: Option<PathBuf>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub builder: BuilderConfig,
    pub file_server: FileServerConfig,
}

pub async fn from_filepath(target_path: &PathBuf) -> Result<Config, String> {
    // get config filepath
    let target_filepath = match env::current_dir() {
        Ok(p) => p.join(target_path),
        _ => return Err("there is no current working directory! interesting!".to_string()),
    };

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
        Err(_e) => return Err("failed to read config from json file".to_string()),
    };
    let mut config: Config = match serde_json::from_str(&json_as_str) {
        Ok(j) => j,
        Err(_e) => return Err("failed to parse config from json string".to_string()),
    };

    let parent_dir = match config_pathbuf.parent() {
        Some(directory) => directory,
        _ => return Err("no parent directory!, crazy!".to_string()),
    };

    config.builder.output_dir = parent_dir.join(config.builder.output_dir);
    config.file_server.directory = parent_dir.join(config.file_server.directory);

    Ok(config)
}
