use config;
use std::env;
use std::path::PathBuf;
use tokio::fs;
use futures::future::try_join_all;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use coyote::Component;
use coyote_html::{pretty_html, Html, Sieve};

use pages;

async fn create_page(name: &str) -> Component {
    match name {
        "home" => pages::home::page(),
        _ => pages::home::page(),
    }
}

async fn write_page(target_filename: &PathBuf, document: String) -> Result<(), std::io::Error> {
    let mut file = match File::create(target_filename).await {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    match file.write_all(document.as_bytes()).await {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    Ok(())
}

async fn generate_pages(config: &config::Config) -> Result<(), std::io::Error> {
    // get state
    // create cached templates
    let curr_dir = match std::env::current_dir() {
        Ok(pb) => pb,
        Err(e) => return Err(e),
    };

    let sieve = Sieve::new();
    let mut html = Html::new();

    // batch process instead of writing each file
    // let mut futures = Vec::new();
    let pretty_sieve = Sieve::new();
    for (name, target_filename) in &config.pages {
        let path = curr_dir.join(target_filename);
        let page = create_page(name).await;
        let document = html.build(&page);
        let pretty_document = pretty_html(&pretty_sieve, &document);

        let parent_path = match path.parent() {
            Some(p) => p,
            _ => &path, // incorrect but to get past current error;
        };

        // get absolte and check if starts with the targt_filepath
        let _ = fs::create_dir_all(parent_path).await;

        // futures.push(write_page(target_filename, document));
        let mut file = match File::create(&path).await {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let result = match file.write_all(pretty_document.as_bytes()).await {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        println!("{:?}", file)
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("{:?}", std::env::current_dir());

    // create config
    let args = match env::args().nth(1) {
        Some(a) => PathBuf::from(a),
        None => return println!("argument error:\nconfig params not found."),
    };
    let config = match config::from_filepath(&args).await {
        Ok(c) => c,
        Err(e) => return println!("config error:\n{}", e),
    };

    let results = generate_pages(&config).await;
    println!("{:?}", &results);
}
