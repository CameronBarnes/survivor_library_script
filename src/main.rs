use humansize::WINDOWS;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking::Client;

fn parse_document_page(str: &str) -> Vec<(&str, usize)> {

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
    }).collect()

}

fn parse_main_page(str: &str) -> Vec<(&str, &str)> {

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("<td style=\"width: \\d+?px;\"><a style=\"font-size: medium;\" title=\".+?\" href=\"(.+?)\">(.+?)</a></td>").unwrap());
    RE.captures_iter(str).map(|c| c.extract()).map(|(_, [path, name])| {
        (name, path)
    }).collect()

}

fn get_page_from_path(path: &str) -> String {

    static MAIN_PATH: &str = "https://www.survivorlibrary.com";
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        reqwest::blocking::ClientBuilder::new()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .build().unwrap()
    });
    CLIENT.get(format!("{MAIN_PATH}{path}")).send().unwrap().text().unwrap()

}

fn main() {

    let fmt = humansize::make_format(WINDOWS);
    
    let total: usize = parse_main_page(&get_page_from_path("/library-download.html")).iter().map(|(name, path)| {
        let total: usize = parse_document_page(&get_page_from_path(path)).iter().map(|(_, size)| size).sum();
        println!("{name}: {}", fmt(total));
        total
    }).sum();

    println!("Total size: {}", fmt(total));

}
