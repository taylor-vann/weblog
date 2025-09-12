/*
    Server for Weblog
    - serve prebuilt static webpages
    - dynamically generate pages from templates
    - (stretch) add auth
    - (stretch) add "blog poster"
*/

use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use std::env;
use std::path::PathBuf;
use tokio::net::TcpListener;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), String> {
    let conf = match get_config().await {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let listener = match TcpListener::bind(&conf.file_server.host_and_port).await {
        Ok(lstnr) => lstnr,
        Err(e) => return Err(e.to_string()),
    };

    println!("file_server: {}", conf.file_server.host_and_port);

    // let svc = service::Svc::from(conf);

    // loop {
    //     let (stream, _remote_address) = match listener.accept().await {
    //         Ok(strm) => strm,
    //         Err(e) => return Err(e.to_string()),
    //     };

    //     let io = TokioIo::new(stream);
    //     let svc = svc.clone();

    //     tokio::task::spawn(async move {
    //         // log service errors here
    //         Builder::new(TokioExecutor::new())
    //             .serve_connection(io, svc)
    //             .await
    //     });
    // }

    Ok(())
}

async fn get_config() -> Result<Config, String> {
    if let Some(conf_path_arg) = env::args().nth(1) {
        let conf_pathbuf = PathBuf::from(conf_path_arg);
        return config::from_filepath(&conf_pathbuf).await;
    }

    Err("arg[1] - filepath is missing".to_string())
}
