use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking::Client;
use crate::types::*;

pub fn parse_document_page(str: &str) -> Vec<Document> {

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("<td.*?><a href=\"(.*?)\">PDF</a> (\\d+) *(.*?)</td>").unwrap());
    RE.captures_iter(str).map(|c| c.extract()).map(|(_, [location, num, unit])| {
        //println!("{location}, {num}, {unit}");
        let mut size = num.parse().unwrap();
        if unit.eq("kb") {
            size *= 1024;
        } else if unit.eq("mb") {
            size *= 1048576;
        }
        (location, size)
    }).map(|(path, size)| {
        let name = path.split_once('.').unwrap().0.to_string();
        Document::new(name, path.to_string(), size)
    }).collect()

}

pub fn parse_main_page(str: &str) -> Vec<(&str, &str)> {

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("<td style=\"width: \\d+?px;\"><a style=\"font-size: medium;\" title=\".+?\" href=\"(.+?)\">(.+?)</a>").unwrap());
    RE.captures_iter(str).map(|c| c.extract()).map(|(_, [path, name])| {
        (name, path)
    }).collect()

}

pub fn get_page_from_path(path: &str) -> String {

    static MAIN_PATH: &str = "https://www.survivorlibrary.com";
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        reqwest::blocking::ClientBuilder::new()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .build().unwrap()
    });
    CLIENT.get(format!("{MAIN_PATH}{path}")).send().unwrap().text().unwrap()

}
