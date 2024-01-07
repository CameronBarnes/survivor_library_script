use humansize::WINDOWS;
use ratatui::{prelude::*, widgets::{ListState, ListItem, List, Block, Borders, Paragraph, ScrollbarState, Scrollbar, ScrollbarOrientation::VerticalRight, Padding, Clear, Wrap, block::Title}};

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

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::new(Direction::Vertical, [
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::new(Direction::Horizontal, [
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

pub fn render(app: &mut App, f: &mut Frame) {

    let vertical = Layout::new(Direction::Vertical, [Constraint::Length(1), Constraint::Length(2), Constraint::Min(0), Constraint::Length(1)]).split(f.size());
    let horizontal = Layout::new(Direction::Horizontal, [Constraint::Percentage(50), Constraint::Percentage(50)]).split(vertical[2]);

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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Title::from("Categories").alignment(Alignment::Left))
        )
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">> ");

    // Render the list
    f.render_stateful_widget(items, horizontal[0], &mut app.category_list.state);
    // Generate scrollbar
    let mut scrollbar_state = ScrollbarState::new(app.categories.len())
        .position(app.category_list.selected());
    //Render scrollbar
    f.render_stateful_widget(Scrollbar::default().orientation(VerticalRight), horizontal[0], &mut scrollbar_state);

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
        .block(Block::default().borders(Borders::ALL).title("Documents").title_alignment(Alignment::Left))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">> ");

    // Render the list
    f.render_stateful_widget(items, horizontal[1], &mut app.doc_list.state);
    // Generate scrollbar
    let mut scrollbar_state = ScrollbarState::new(app.categories[selected_category].documents.len())
        .position(app.doc_list.selected());
    //Render scrollbar
    f.render_stateful_widget(Scrollbar::default().orientation(VerticalRight), horizontal[1], &mut scrollbar_state);

    //Render the title
    f.render_widget(
        Paragraph::new("Survivor Library Dowload Tool")
            .bold()
            .alignment(Alignment::Center),
        vertical[0]
    );

    // Render instructions
    f.render_widget(
        Paragraph::new("ESC or ctrl-C to quit | arrow keys for navigation | space to toggle item | ENTER to download | ctrl-A to reset | 'S' to change sort mode")
            .alignment(Alignment::Center)
            .wrap(Wrap{trim: true}),
        vertical[1]
    );

    // Render the total
    // Calculate total
    let total: usize = app.categories.iter().map(|cat| cat.enabled_size()).sum();
    let total_size_text = format!("Total Enabled Size: {}", humansize::format_size(total, WINDOWS));
    f.render_widget(
        Paragraph::new(total_size_text.clone())
            .bold()
            .alignment(Alignment::Center),
        vertical[3]
    );

    // Render the popup here
    if app.download {
        let area = centered_rect(60, 60, f.size());
        f.render_widget(Clear, area); // Clear the area so we can render over it
        let paragraph = Paragraph::new(format!("{total_size_text}\n\nPress ESC or ctrl-C to go back\nENTER to download files now\npress 'P' to print the file paths to a file"))
            .bold()
            .alignment(Alignment::Center)
            .wrap(Wrap{trim: false})
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Download")
                    .title_alignment(Alignment::Center)
                    .title_style(Style::default().bold())
                    .padding(Padding::new(5, 10, 1, 2))
            );

        // Render
        f.render_widget(paragraph, area);
    }
}
