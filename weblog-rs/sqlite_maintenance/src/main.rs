// takes args input like

// weblog_sqlite -- setup
// weblog_sqlite -- clean
// weblog_sqlite -- article

// right now only setup
// then create tables from sqlite interface

// right now people, roles, roles_people

use std::env;
use std::path::PathBuf;

use config;

#[tokio::main]
async fn main() -> Result<(), String> {
    // get args 0
    let action = match env::args().nth(0) {
        Some(actn) => actn,
        _ => return Err("no action found at arg[0]".to_string()),
    };

    match action.as_str() {
        "setup_dbs" => setup_dbs().await,
        _ => Err("no action function matched arg[0]".to_string()),
    }
}

async fn setup_dbs() -> Result<(), String> {
    let config_path_buf = match env::args().nth(1) {
        Some(fbjs) => PathBuf::from(fbjs),
        _ => return Err("arg[1] config path not included.".to_string()),
    };

    let fallback_path_buf = match env::args().nth(1) {
        Some(fbjs) => PathBuf::from(fbjs),
        _ => return Err("arg[1] fallback path not included.".to_string()),
    };

    let config = match config::Config::from_filepath(&config_path_buf).await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let fallback_user = match config::FallbackUser::from_filepath(&fallback_path_buf).await {
        Ok(fu) => fu,
        Err(e) => return Err(e),
    };

    Ok(())
}
