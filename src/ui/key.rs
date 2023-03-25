use crossterm::event::{KeyEvent, KeyCode};

use super::state::{AppState, Page};

#[derive(PartialEq)]
pub enum KeyBindEvent {
    None,
    Quit,
}

impl KeyBindEvent {
    pub fn is_none(self) -> bool {
        self == KeyBindEvent::None
    }

    pub fn is_quit(self) -> bool {
        self == KeyBindEvent::Quit
    }
}

pub fn handle_key(app: &mut AppState, event: KeyEvent) -> KeyBindEvent {
    if handle_general_key(app, event).is_quit() {
        return KeyBindEvent::Quit;
    }

    match app.page {
        Page::Search => handle_search_key(app, event),
        Page::Board => handle_board_key(app, event),
        Page::Post => handle_post_key(app, event),
    }
}

fn handle_general_key(mut app: &mut AppState, event: KeyEvent) -> KeyBindEvent {
    match event.code {
        KeyCode::Char('q') => KeyBindEvent::Quit,
        _ => KeyBindEvent::None,
    }
}

fn handle_search_key(mut app: &mut AppState, event: KeyEvent) -> KeyBindEvent {
    KeyBindEvent::None
}

fn handle_board_key(mut app: &mut AppState, event: KeyEvent) -> KeyBindEvent {
    KeyBindEvent::None
}

fn handle_post_key(mut app: &mut AppState, event: KeyEvent) -> KeyBindEvent {
    KeyBindEvent::None
}