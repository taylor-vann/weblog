use coyote::{Component, Html};
use std::path;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use config;
use pages;

pub async fn generate_pages(config: &config::Config) -> Result<(), String> {
    let mut html = Html::new();

    println!("about to generate pages");

    for (name, page_path) in &config.pages {
        let page_component = match create_page(name) {
            Some(p) => p,
            _ => continue,
        };

        let page = match html.build(&page_component) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        if let Err(e) = write_page(&config.directory, &page_path, &page).await {
            return Err(e);
        }
    }

    Ok(())
}

fn create_page(name: &str) -> Option<Component> {
    let page = match name {
        "home" => pages::pages::home::page(),
        _ => return None,
    };

    Some(page)
}

async fn write_page(
    directory: &PathBuf,
    target_filename: &PathBuf,
    page: &str,
) -> Result<(), String> {
    let target_file_abs = match path::absolute(directory.join(target_filename)) {
        Ok(t) => t,
        _ => return Err("Failed to create absolute directories for target path".to_string()),
    };

    if !target_file_abs.starts_with(directory) {
        return Err("page filename is outside the bounds of top level directory".to_string());
    }

    // check if belongs in www/
    let mut file = match File::create(target_file_abs).await {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    match file.write_all(page.as_bytes()).await {
        Ok(_file) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

// async fn copy_dir(config: &config::Config) -> Result<(), std::io::Error> {
//     if let Ok(mut dir_copy) = DirCopy::try_from_path(&config.origin_dir, &config.target_dir).await {
//         while let Some((source_path, target_path)) = dir_copy.next_entry().await {
//             if source_path.is_file() {
//                 if let Err(e) = fs::copy(&source_path, &target_path).await {
//                     return Err(e);
//                 };
//             }

//             if source_path.is_dir() {
//                 if let Err(e) = fs::create_dir_all(&target_path).await {
//                     return Err(e);
//                 }
//             }
//         }
//     };

//     Ok(())
// }

// async fn create_target_dir(config: &config::Config) -> Result<(), String> {
//     let target_dir_exists = match config.target_dir.try_exists() {
//         Ok(yn) => yn,
//         _ => return Err("failed to determine if config.target_dir exists.".to_string()),
//     };

//     if !target_dir_exists {
//         if let Err(_e) = fs::create_dir_all(&config.target_dir).await {
//             return Err("failed to create config.target_dir".to_string());
//         };
//     }

//     Ok(())
// }
