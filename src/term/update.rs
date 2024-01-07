use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
    if !app.download {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit()
                }
            }
            KeyCode::Right => app.right(),
            KeyCode::Left => app.left(),
            KeyCode::Up => app.previous(),
            KeyCode::Down => app.next(),
            KeyCode::Char(' ') => app.toggle(),
            KeyCode::Enter => app.download = true,
            KeyCode::Tab => app.toggle_all(),
            KeyCode::Char('a') | KeyCode::Char('A') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.reset();
                }
            },
            KeyCode::Char('s') | KeyCode::Char('S') => {
                app.toggle_sort_style();
            },
            KeyCode::Home => app.home(),
            KeyCode::End => app.end(),
            _ => {}
        };
    } else {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => app.download = false,
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.download = false;
                }
            },
            KeyCode::Enter => app.quit(),
            KeyCode::Char('p') | KeyCode::Char('P') => {
                app.download = false;
                app.print = true;
                app.quit();
            }
            _ => {},
        }
    }
}
