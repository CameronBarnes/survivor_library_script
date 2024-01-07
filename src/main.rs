mod parsing;
mod types;

use humansize::WINDOWS;
use parsing::{parse_main_page, get_page_from_path, parse_document_page};
use types::Category;

fn get_categories() -> Vec<Category> {
    parse_main_page(&get_page_from_path("/library-download.html")).iter().map(|(name, path)| {
        let documents = parse_document_page(&get_page_from_path(path));
        Category::new(name.to_string(), documents)
    }).collect()
}

fn main() {

    let categories = get_categories();
    let total: usize = categories.iter().map(|cat| {
        let size = cat.size(false);
        println!("{}: {}", cat.name, size);
        size
    }).sum();

    println!("Total Size: {}", humansize::format_size(total, WINDOWS));

}
