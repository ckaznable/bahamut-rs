use std::sync::mpsc::Sender;

use crossterm::event::{KeyEvent, KeyCode};

use crate::channel::DataRequestMsg;

use super::state::{AppState, Page};

#[derive(PartialEq)]
pub enum KeyBindEvent {
    None,
    Quit,
}

impl KeyBindEvent {
    pub fn is_quit(self) -> bool {
        self == KeyBindEvent::Quit
    }
}

pub fn handle_key(app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    if handle_general_key(app, event, tx.clone()).is_quit() {
        return KeyBindEvent::Quit;
    }

    match app.page {
        Page::Search => handle_search_key(app, event, tx.clone()),
        Page::Board => handle_board_key(app, event, tx.clone()),
        Page::Post => handle_post_key(app, event, tx.clone()),
    }
}

fn handle_general_key(mut app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    match event.code {
        KeyCode::Char('q') => return KeyBindEvent::Quit,
        KeyCode::Char('a') => tx.send(DataRequestMsg::SearchResult(String::from("寶可夢"))),
        _ => return KeyBindEvent::None,
    };

    KeyBindEvent::None
}

fn handle_search_key(mut app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    KeyBindEvent::None
}

fn handle_board_key(mut app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    KeyBindEvent::None
}

fn handle_post_key(mut app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    KeyBindEvent::None
}
