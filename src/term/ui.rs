use humansize::WINDOWS;
use ratatui::{prelude::*, widgets::{ListState, ListItem, List, Block, Borders, Paragraph}};

use super::app::App;

#[derive(Debug)]
pub struct StatefulListCounter {
    state: ListState,
    size: usize,
}

impl StatefulListCounter {
    pub fn new(size: usize) -> Self {
        StatefulListCounter{state: ListState::default(), size}
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.size - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.size - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected(&mut self) -> usize {
        match self.state.selected() {
            Some(i) => i,
            None => {
                self.state.select(Some(0));
                0
            }
        }
    }

    pub fn set_selected(&mut self, index: usize) {
        let index = usize::min(self.size - 1, index);
        self.state.select(Some(index));
    }
}

pub fn render(app: &mut App, f: &mut Frame) {

    let vertical = Layout::new(Direction::Vertical, [Constraint::Min(0), Constraint::Length(1)]).split(f.size());
    let horizontal = Layout::new(Direction::Horizontal, [Constraint::Percentage(50), Constraint::Percentage(50)]).split(vertical[0]);

    // Collect and format the category list items
    let items: Vec<ListItem> = app.categories.iter().map(|category| {
        let name = &category.name;
        let size = category.human_readable_size(true);
        let mut item = ListItem::new(format!("{name}:  {size}"));
        if !category.enabled {
            item = item.style(Style::default().dim());
        }
        item
    }).collect();
    // Build the list
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Categories"))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">> ");

    // Render the list
    f.render_stateful_widget(items, horizontal[0], &mut app.category_list.state);

    // Collect and format the category list items
    let selected_category = app.category_list.selected();
    let items: Vec<ListItem> = app.categories[selected_category].documents.iter().map(|doc| {
        let name = &doc.name;
        let size = doc.human_readable_size();
        let mut item = ListItem::new(format!("{name}:  {size}"));
        if !doc.enabled {
            item = item.style(Style::default().dim());
        }
        item
    }).collect();
    // Build the list
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Documents"))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">> ");

    // Render the list
    f.render_stateful_widget(items, horizontal[1], &mut app.doc_list.state);

    // Render the total
    // Calculate total
    let total: usize = app.categories.iter().map(|cat| cat.enabled_size()).sum();
    f.render_widget(Paragraph::new(format!("Total Enabled Size: {}", humansize::format_size(total, WINDOWS))).bold(), vertical[1]);
}
