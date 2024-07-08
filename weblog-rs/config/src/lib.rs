use serde::{Deserialize, Serialize};
use serde_json;
use tokio::fs;

use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub pages: Vec<(String, PathBuf)>,
}

pub async fn from_filepath(filepath: &PathBuf) -> Result<Config, String> {
    // get position relative to working directory
    let (curr_dir, config_pathbuff) = match std::env::current_dir() {
        Ok(pb) => (pb.clone(), pb.join(filepath)),
        Err(e) => return Err(e.to_string()),
    };

    let parent_dir = match config_pathbuff.parent() {
        Some(directory) => directory,
        _ => return Err("no parent directory!, crazy!".to_string()),
    };

    println!("{:?}", parent_dir);

    // build json conifg
    let json_as_str = match fs::read_to_string(&config_pathbuff).await {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    let config: Config = match serde_json::from_str(&json_as_str) {
        Ok(j) => j,
        Err(e) => return Err(e.to_string()),
    };

    let mut pages: Vec<(String, PathBuf)> = Vec::new();
    for (name, address) in &config.pages {
        // get parent then join on parent of config
        let path = parent_dir.join(address);

        pages.push((name.to_string(), path));
    }

    Ok(Config { pages: pages })
}
