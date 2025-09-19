use config;
use std::env;
use std::path;

mod pages;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return Err("invalid arguments".to_string());
    }

    let config_path_str = match args.last() {
        Some(config_path) => config_path,
        _ => return Err("config path not found".to_string()),
    };

    let build_steps = &args[1..args.len() - 1].join(":");
    match build_steps.as_str() {
        "build:all" => build_all(config_path_str).await,
        _ => Err("invalid arguments".to_string()),
    }
}

async fn build_all(config_path_str: &str) -> Result<(), String> {
    let config = match config::from_filepath(&path::PathBuf::from(&config_path_str)).await {
        Ok(cnf) => cnf,
        Err(e) => return Err(e),
    };

    // generate posts
    pages::generate_pages(&config).await
}
