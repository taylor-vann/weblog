use std::env;
use std::path;

use config;

mod pages;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("args {:?}", env::args());

    // args
    // get build pages ./config for pages
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

    pages::generate_pages(&config).await
}

// #[tokio::main]
// async fn main() {

//     // weblog setup all ./config.json
//     // weblog build all ./config.json
//     // weblog build posts ./
//     // weblog update all
//     //
//     //
//     // create config
//     let args = match env::args().nth(1) {
//         Some(a) => PathBuf::from(a),
//         None => return println!("argument error:\nconfig params not found."),
//     };

//     let config = match config::from_filepath(&args).await {
//         Ok(c) => c,
//         Err(e) => return println!("config error:\n{}", e),
//     };

//     if let Err(e) = create_target_dir(&config).await {
//         println!("{}", e);
//     };

//     if let Err(e) = copy_dir(&config).await {
//         println!("{}", e);
//     };

//     if let Err(e) = generate_pages(&config).await {
//         println!("{}", e);
//     };
// }
