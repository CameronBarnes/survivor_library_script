use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
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
        _ => {}
    };
}
