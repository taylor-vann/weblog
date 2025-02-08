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
use sqlite_interface::{ip_rate_limits, people, roles, roles_to_people, sessions};

#[tokio::main]
async fn main() -> Result<(), String> {
    // get args 0
    let action = match env::args().nth(1) {
        Some(actn) => actn,
        _ => return Err("no action found at arg[1]".to_string()),
    };

    let config_path_buf = match env::args().nth(2) {
        Some(fbjs) => PathBuf::from(fbjs),
        _ => return Err("arg[2] config path not included.".to_string()),
    };
    println!("{:?}", config_path_buf);

    let config = match Config::from_filepath(&config_path_buf).await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let results = match action.as_str() {
        "setup_dbs" => setup_dbs(&config).await,
        "create_fallback_account" => create_fallback_account(&config).await,
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

    Ok(())
}

async fn create_fallback_account(config: &Config) -> Result<(), String> {
    println!("create_fallback_account()");
    let fallback_path_buf = match env::args().nth(3) {
        Some(fbjs) => PathBuf::from(fbjs),
        _ => return Err("arg[3] fallback path not included.".to_string()),
    };

    let fallback_account = match FallbackUser::from_filepath(&fallback_path_buf).await {
        Ok(fu) => fu,
        Err(e) => return Err(e),
    };

    println!("{:?}", fallback_account);

    // let hashed = people::create(
    //     &fallback_account.email,
    //     &fallback_account.screen_name,
    //     &fallback_account.password,
    // );

    // println!("{:?}", hashed);

    // create user and password
    // add roles

    Ok(())
}
