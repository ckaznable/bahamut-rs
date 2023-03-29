use std::sync::mpsc::Sender;

use crossterm::event::{KeyEvent, KeyCode, Event};
use tui_input::backend::crossterm::EventHandler;

use crate::channel::DataRequestMsg;

use super::state::{AppState, Page, CursorMoveable, InputMode};

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

    if app.loading {
        return KeyBindEvent::None;
    }

    match app.page {
        Page::Search => handle_search_key(app, event, tx.clone()),
        Page::Board => handle_board_key(app, event, tx.clone()),
        Page::Post => handle_post_key(app, event, tx.clone()),
    }
}

fn handle_general_key(app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    match event.code {
        KeyCode::Char('q') => {
            match app.page {
                Page::Search => return KeyBindEvent::Quit,
                Page::Board => app.page = Page::Search,
                Page::Post => app.page = Page::Board,
            }
        },
        _ => ()
    };

    KeyBindEvent::None
}

fn handle_search_key(app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    match app.search.mode {
        InputMode::Normal => match event.code {
            KeyCode::Char('j') | KeyCode::Down => app.search.next(),
            KeyCode::Char('k') | KeyCode::Up => app.search.previous(),
            KeyCode::Char('a' |'e' | 'i' | 'o') => app.search.mode(InputMode::Edit),
            KeyCode::Enter => {
                if let Some(i) = app.search.state.selected() {
                    if let Some(board) = app.search.items.get(i) {
                        app.loading = true;
                        app.board.name(board.name.to_owned());
                        app.board.id(board.id.to_owned());
                        tx.send(DataRequestMsg::BoardPage(board.id.to_string(), 1)).map_or(() , |_|());
                    }
                }
            }
            _ => ()
        }
        InputMode::Edit => match event.code {
            KeyCode::Esc => app.search.mode(InputMode::Normal),
            KeyCode::Enter => {
                app.search.mode(InputMode::Normal);
                app.loading = true;
                tx.send(DataRequestMsg::SearchResult(app.search.input.value().into())).map_or((), |_|());
            }
            _ => {
                app.search.input.handle_event(&Event::Key(event));
            }
        },
        InputMode::Search => todo!(),
    }

    KeyBindEvent::None
}

fn handle_board_key(app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    match event.code {
        KeyCode::Char('j') | KeyCode::Down => app.board.next(),
        KeyCode::Char('k') | KeyCode::Up => app.board.previous(),
        _ => (),
    }

    KeyBindEvent::None
}

fn handle_post_key(mut app: &mut AppState, event: KeyEvent, tx: Sender<DataRequestMsg>) -> KeyBindEvent {
    KeyBindEvent::None
}
