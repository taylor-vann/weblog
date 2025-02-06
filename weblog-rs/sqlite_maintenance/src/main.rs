// takes args input like

// weblog_sqlite -- setup
// weblog_sqlite -- clean
// weblog_sqlite -- article

// right now only setup
// then create tables from sqlite interface

// right now people, roles, roles_people

use std::env;
use std::path::PathBuf;

use config::{Config, FallbackUser};
use rusqlite::{Connection, Result};
use sqlite_interface::{
    ip_rate_limits, people, roles, roles_to_people, session_rate_limits, sessions,
};

#[tokio::main]
async fn main() -> Result<(), String> {
    // get args 0
    let action = match env::args().nth(1) {
        Some(actn) => actn,
        _ => return Err("no action found at arg[0]".to_string()),
    };

    let config_path_buf = match env::args().nth(2) {
        Some(fbjs) => PathBuf::from(fbjs),
        _ => return Err("arg[1] config path not included.".to_string()),
    };
    println!("{:?}", config_path_buf);

    let config = match Config::from_filepath(&config_path_buf).await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let results = match action.as_str() {
        "setup_dbs" => setup_dbs(&config).await,
        "create_fallback_user" => create_fallback_user(&config).await,
        _ => return Err("no action function matched arg[0]".to_string()),
    };

    if let Err(e) = results {
        println!("{:?}", e);
    }

    Ok(())
}

async fn setup_dbs(config: &Config) -> Result<(), String> {
    if let Err(e) = people::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = roles::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = roles_to_people::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = sessions::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = ip_rate_limits::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    if let Err(e) = session_rate_limits::create_table(&config.sqlite_db_auth) {
        return Err(e.to_string());
    };

    Ok(())
}

async fn create_fallback_user(config: &Config) -> Result<(), String> {
    let fallback_path_buf = match env::args().nth(1) {
        Some(fbjs) => PathBuf::from(fbjs),
        _ => return Err("arg[1] fallback path not included.".to_string()),
    };

    let fallback_user = match FallbackUser::from_filepath(&fallback_path_buf).await {
        Ok(fu) => fu,
        Err(e) => return Err(e),
    };

    Ok(())
}
