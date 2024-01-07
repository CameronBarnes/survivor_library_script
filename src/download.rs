use core::panic;
use std::{path::Path, fs::{self, File}, io::Write, cmp::min};

use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

use crate::types::Category;

fn setup_folder(path_str: &str) -> Result<()> {
    let path = Path::new(path_str);
    if path.exists() {
        if path.is_file() {
            panic!("Cannot create {path_str} folder as a file exists in its place");
        } else {
            return Ok(());
        }
    } else {
        fs::create_dir(path)?
    }

    Ok(())

}

pub fn setup_folders() -> Result<()> {
    setup_folder("./survivor_library")
}

// Code sourced from https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
pub async fn download_file(client: &reqwest::Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    
    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {}", url));

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file"))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file"))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {} to {}", url, path));
    Ok(())
}

pub fn handle_download_file(url: &str, path_str: &str, overwrite: bool) -> Result<()> {

    static CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
        reqwest::ClientBuilder::new()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .build().unwrap()
    });

    let path = Path::new(path_str);
    if path.exists() {
        if path.is_dir() {
            panic!("Cant download file {path_str}, as a folder exists in its place");
        } else if !overwrite { // If we're not overwriting, then we assume this file is done
            return Ok(());
        } else { // We're overwriting an existing file, so we need to delete the old one first
            fs::remove_file(path)?;
        }
    }

    static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

    match RT.block_on(download_file(&CLIENT, url, path_str)).err() {
        Some(err) => Err(anyhow!("{err}")),
        None => Ok(()),
    }

}

pub fn download_category(category: Category) -> Result<()> {

    let mut path = String::from("./survivor_library/");
    path.push_str(&category.name);
    setup_folder(&path)?;
    category.documents.iter().filter(|doc| doc.enabled).for_each(|doc| {
        let path = format!("{path}/{}", doc.path.strip_prefix("/library/").unwrap());
        handle_download_file(&doc.get_url(), &path, false).unwrap(); // We're just going to ignore this for now
    });

    Ok(())

}
