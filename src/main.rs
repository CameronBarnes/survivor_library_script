mod parsing;
mod types;

use parsing::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use types::{Category, LibraryItem};

static IS_WINDOWS: bool = cfg!(windows);

fn get_categories() -> Vec<LibraryItem> {
    parse_main_page(&get_page_from_path("/library-download.html")).par_iter().map(|(name, path)| {
        let documents = parse_document_page(&get_page_from_path(path));
        LibraryItem::Category(Category::new(name.to_string(), documents, false))
    }).collect()
}

fn main() {

    let category = LibraryItem::Category(Category::new("Survivor Library".to_string(), get_categories(), false));
    println!("{}", serde_json::to_string(&category).unwrap());

}
