use crate::types::{Document, DownloadType, LibraryItem};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking::Client;

pub fn parse_document_page(str: &str) -> Vec<LibraryItem> {
    static MAIN_PATH: &str = "https://www.survivorlibrary.com";
    static RE: Lazy<Regex> =
        Lazy::new(|| Regex::new("<td.*?><a href=\"(.*?)\">PDF</a> (\\d+) *(.*?)</td>").unwrap());

    RE.captures_iter(str)
        .map(|c| c.extract())
        .map(|(_, [location, num, unit])| {
            //println!("{location}, {num}, {unit}");
            let mut size = num.parse().unwrap();
            if unit.eq("kb") {
                size *= 1024;
            } else if unit.eq("mb") {
                size *= 1_048_576;
            }
            (location, size)
        })
        .map(|(path, size)| {
            let name = path
                .strip_prefix("/library/")
                .unwrap()
                .split_once('.')
                .unwrap()
                .0
                .replace('_', " ");
            let url = format!("{MAIN_PATH}{path}");
            LibraryItem::Document(Document::new(name, url, size, DownloadType::Http))
        })
        .collect()
}

pub fn parse_main_page(str: &str) -> Vec<(&str, &str)> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new("<td style=\"width: \\d+?px;\"><a style=\"font-size: medium;\" title=\".+?\" href=\"(.+?)\">(.+?)</a>").unwrap()
    });
    RE.captures_iter(str)
        .map(|c| c.extract())
        .map(|(_, arr)| arr.into())
        .collect()
}

pub fn get_page_from_path(path: &str) -> String {
    static MAIN_PATH: &str = "https://www.survivorlibrary.com";
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        reqwest::blocking::ClientBuilder::new()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .build()
            .unwrap()
    });
    CLIENT
        .get(format!("{MAIN_PATH}/{path}"))
        .send()
        .unwrap()
        .text()
        .unwrap()
}
