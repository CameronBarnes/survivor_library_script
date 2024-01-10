mod parsing;
mod types;
mod term;
mod download;

use std::path::Path;
use std::fs;

use anyhow::Result;

use indicatif::ParallelProgressIterator;
use parsing::{parse_main_page, get_page_from_path, parse_document_page};
use ratatui::{backend::CrosstermBackend, Terminal};
use rayon::prelude::*;
use term::{app::App, event::EventHandler, tui::Tui, update::update};
use types::Category;

fn get_categories() -> Vec<Category> {
    parse_main_page(&get_page_from_path("/library-download.html")).par_iter().progress().map(|(name, path)| {
        let documents = parse_document_page(&get_page_from_path(path));
        Category::new(name.to_string(), documents)
    }).collect()
}

fn main() -> Result<()> {

    println!("Getting Data from library...");
    let categories = get_categories();
    println!("Data Retrieved");
    
    let mut app = App::new(categories);

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;
    
    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            term::event::Event::Tick => {},
            term::event::Event::Key(key_event) => update(&mut app, key_event),
            term::event::Event::Mouse(_) => {},
            term::event::Event::Resize(_, _) => {},
            term::event::Event::FocusGained |
                term::event::Event::FocusLost => {}, // Do nothing, needed for windows support
        }
    }

    tui.exit()?;

    // Now we handle downloading the files or saving the paths
    if app.download {
        download::setup_folders()?;
        for category in app.categories.into_iter().filter(|cat| cat.enabled) {
            println!("Downloading {} from {}", category.human_readable_size(true), &category.name);
            download::download_category(category)?;
        }
    } else if app.print {
        let path = Path::new("./survivor_library_paths.txt");
        let mut output = String::new();
        app.categories.iter().filter(|cat| cat.enabled)
            .flat_map(|cat| cat.documents.iter().filter(|doc| doc.enabled)).for_each(|doc| {
            output.push_str(&doc.get_url());
            output.push('\n');
        });
        fs::write(path, output)?;
    }

    Ok(())

}
