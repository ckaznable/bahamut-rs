use std::{cell::RefCell, rc::Rc, sync::mpsc::Sender};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

use crate::channel::DataRequestMsg;

use super::state::{AppState, CursorMoveable, InputMode, Page};

#[derive(PartialEq)]
pub enum KeyBindEvent {
    None,
    Quit,
}

impl KeyBindEvent {
    pub fn is_quit(&self) -> bool {
        *self == KeyBindEvent::Quit
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
        Page::Search => handle_search_key(app, event, tx),
        Page::Board => handle_board_key(app, event, tx),
        Page::Post => handle_post_key(app, event, tx),
        Page::Comment => handle_comment_key(app, event, tx),
    }
}

fn handle_general_key(
    app: &mut AppState,
    event: KeyEvent,
    _: Sender<DataRequestMsg>,
) -> KeyBindEvent {
    if let KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        kind: _,
        state: _,
    } = event
    {
        return KeyBindEvent::Quit;
    }

    if let KeyCode::Char('q') = event.code {
        if app.search.mode != InputMode::Edit {
            match app.page {
                Page::Search => return KeyBindEvent::Quit,
                Page::Board => app.page = Page::Search,
                Page::Post => app.page = Page::Board,
                Page::Comment => app.page = Page::Post,
            }
        }
    };

    KeyBindEvent::None
}

fn handle_search_key(
    app: &mut AppState,
    event: KeyEvent,
    tx: Sender<DataRequestMsg>,
) -> KeyBindEvent {
    match app.search.mode {
        InputMode::Normal => match event.code {
            KeyCode::Char('j') | KeyCode::Down => app.search.next(),
            KeyCode::Char('k') | KeyCode::Up => app.search.previous(),
            KeyCode::Char('a' | 'e' | 'i' | 'o') => app.search.mode(InputMode::Edit),
            KeyCode::Enter => {
                if let Some(i) = app.search.state.selected() {
                    if let Some(board) = app.search.items.get(i) {
                        app.loading = true;
                        app.board.name(board.name.to_owned());
                        app.board.id(board.id.to_owned());
                        tx.send(DataRequestMsg::BoardPage(board.id.to_string(), 1, true))
                            .map_or((), |_| ());
                    }
                }
            }
            _ => (),
        },
        InputMode::Edit => match event.code {
            KeyCode::Esc => app.search.mode(InputMode::Normal),
            KeyCode::Enter => {
                app.search.mode(InputMode::Normal);
                let value = app.search.input.value();
                if !value.is_empty() {
                    app.loading = true;
                    tx.send(DataRequestMsg::SearchResult(value.into()))
                        .map_or((), |_| ());
                }
            }
            _ => {
                app.search.input.handle_event(&Event::Key(event));
            }
        },
    }

    KeyBindEvent::None
}

fn handle_board_key(
    app: &mut AppState,
    event: KeyEvent,
    tx: Sender<DataRequestMsg>,
) -> KeyBindEvent {
    match event.code {
        KeyCode::Char('j') | KeyCode::Down => app.board.next(),
        KeyCode::Char('k') | KeyCode::Up => app.board.previous(),
        KeyCode::Char('h') | KeyCode::Left => {
            if app.board.page <= 1 {
                app.board.page(1)
            } else {
                app.loading = true;
                tx.send(DataRequestMsg::BoardPage(
                    app.board.id.to_owned(),
                    app.board.page - 1,
                    true,
                ))
                .map_or((), |_| ())
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if app.board.page >= app.board.last_page {
                app.board.page(app.board.last_page)
            } else {
                app.loading = true;
                tx.send(DataRequestMsg::BoardPage(
                    app.board.id.to_owned(),
                    app.board.page + 1,
                    true,
                ))
                .map_or((), |_| ())
            }
        }
        KeyCode::Enter => {
            if let Some(v) = app.board.state.selected() {
                if let Some(post) = app.board.items.get(v) {
                    app.loading = true;
                    app.post.url = post.url.to_string();
                    tx.send(DataRequestMsg::PostPage(post.url.to_string(), 1, true))
                        .map_or((), |_| ())
                }
            }
        }
        KeyCode::Char('r') => {
            app.loading = true;
            tx.send(DataRequestMsg::BoardPage(
                app.board.id.to_owned(),
                app.board.page,
                false,
            ))
            .map_or((), |_| ())
        }
        _ => (),
    }

    KeyBindEvent::None
}

fn handle_post_key(
    app: &mut AppState,
    event: KeyEvent,
    tx: Sender<DataRequestMsg>,
) -> KeyBindEvent {
    let app = Rc::new(RefCell::new(app));
    let next = |app: &Rc<RefCell<&mut AppState>>| {
        let mut app = app.borrow_mut();
        if app.post.next().is_none() && app.post.has_next() {
            app.loading = true;
            tx.send(DataRequestMsg::PostPage(
                app.post.url.to_owned(),
                app.post.page + 1,
                false,
            ))
            .map_or((), |_| ());
        }
    };

    let app = Rc::clone(&app);
    match event.code {
        KeyCode::PageDown => next(&app),
        KeyCode::PageUp => app.borrow_mut().post.previous(),
        KeyCode::Home => app.borrow_mut().post.first(),
        KeyCode::Char('j') | KeyCode::Down => app.borrow_mut().post.scroll_down(),
        KeyCode::Char('k') | KeyCode::Up => app.borrow_mut().post.scroll_up(),
        KeyCode::Char('o') => {
            let mut app = app.borrow_mut();
            app.loading = true;
            app.comment.init();
            if let Some(content) = app.post.current() {
                tx.send(DataRequestMsg::CommentPage(
                    app.board.id.to_owned(),
                    content.id.to_owned(),
                ))
                .map_or((), |_| ());
            }
        }
        KeyCode::Char('r') => {
            let mut app = app.borrow_mut();
            app.loading = true;
            tx.send(DataRequestMsg::PostPage(
                app.post.url.to_owned(),
                app.post.page,
                false,
            ))
            .map_or((), |_| ())
        }
        _ => (),
    };

    // with control
    if let KeyEvent {
        code,
        modifiers: KeyModifiers::CONTROL,
        kind: _,
        state: _,
    } = event
    {
        match code {
            KeyCode::Char('f') => next(&app),
            KeyCode::Char('b') => app.borrow_mut().post.previous(),
            _ => (),
        }
    };

    KeyBindEvent::None
}

fn handle_comment_key(
    app: &mut AppState,
    event: KeyEvent,
    _: Sender<DataRequestMsg>,
) -> KeyBindEvent {
    match event.code {
        KeyCode::Char('j') | KeyCode::Down => app.comment.next(),
        KeyCode::Char('k') | KeyCode::Up => app.comment.previous(),
        _ => (),
    };

    KeyBindEvent::None
}
