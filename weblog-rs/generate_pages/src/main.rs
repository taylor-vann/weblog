use coyote::Component;
use coyote_html::{Html, ServerRules};
use std::env;
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use walk_directory::DirCopy;

use config;
use pages;

const PAGE_FILEPATHS: [&'static (&str, &str); 1] = [&("home", "index.html")];

async fn create_page(name: &str) -> Option<Component> {
    let page = match name {
        "home" => pages::home::page(),
        _ => return None,
    };

    Some(page)
}

async fn write_page(target_filename: &PathBuf, document: &str) -> Result<(), std::io::Error> {
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
    // copy origin directory
    if let Ok(mut dir_copy) = DirCopy::try_from_path(&config.origin_dir, &config.target_dir).await {
        if let Some((source_path, target_path)) = dir_copy.next_entry().await {
            if let Err(e) = fs::copy(source_path, target_path).await {
                return Err(e);
            };
        }
    };

    // iterate through pages
    let rules = ServerRules::new();
    let mut html = Html::new();

    for (name, path_str) in PAGE_FILEPATHS {
        let page = match create_page(name).await {
            Some(p) => p,
            _ => continue,
        };

        let document = html.build(&rules, &page);

        let page_path = config.target_dir.join(path_str);
        if let Err(e) = write_page(&page_path, &document).await {
            return Err(e);
        }
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

    if let Err(e) = generate_pages(&config).await {
        println!("{:?}", e.to_string());
    };
}
