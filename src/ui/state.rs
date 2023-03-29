use std::sync::mpsc::Sender;

use bahamut::api::{search::SearchResult, board::{BoardPost, BoardPage}};
use ratatui::widgets::ListState;
use tui_input::Input;

use crate::channel::DataRequestMsg;

#[derive(Clone)]
pub enum InputMode {
    Normal,
    Edit,
    Search,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
    }
}

pub enum Page {
    Search,
    Board,
    Post,
}

pub trait CursorMoveable {
    fn index(&self) -> usize;
    fn max(&self) -> usize;
    fn next(&mut self);
    fn previous(&mut self);

    fn next_index(&self) -> usize {
        let i = self.index();
        if i >= self.max() - 1 {
            0
        } else {
            i + 1
        }
    }

    fn previous_index(&self) -> usize {
        let i = self.index();
        let max = self.max();

        if i > 0 {
            return i - 1
        }

        // i == 0
        if max > 0 {
            max - 1
        } else {
            i
        }
    }
}

pub trait ListStateInit<T> {
    fn lists(&self) -> &Vec<T>;
    fn state(&mut self) -> &mut ListState;

    fn init_select(&mut self) {
        if self.lists().len() > 0 {
            self.state().select(Some(0));
        } else {
            self.state().select(None);
        }
    }
}

pub struct AppState {
    pub page: Page,
    pub search: SearchPageState,
    pub board: BoardPageState,
    pub loading: bool,

    tx: Sender<DataRequestMsg>,
}

impl AppState {
    pub fn new(tx: Sender<DataRequestMsg>) -> AppState {
        AppState {
            page: Page::Search,
            search: SearchPageState::default(),
            board: BoardPageState::default(),
            loading: false,
            tx,
        }
    }

    pub fn get_page(&self) -> &dyn CursorMoveable {
        match self.page {
            Page::Search => &self.search,
            Page::Board => &self.board,
            _ => todo!()
        }
    }
}

#[derive(Default, Clone)]
pub struct SearchPageState {
    pub state: ListState,
    pub items: Vec<SearchResult>,
    pub mode: InputMode,
    pub input: Input,
}

impl SearchPageState {
    pub fn items(&mut self, items: Vec<SearchResult>) {
        self.items = items;
    }

    pub fn mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }
}

impl ListStateInit<SearchResult> for SearchPageState {
    fn lists(&self) -> &Vec<SearchResult> {
        &self.items
    }

    fn state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl CursorMoveable for SearchPageState {
    fn index(&self) -> usize {
        self.state.selected().map_or(1, |x|x)
    }

    fn max(&self) -> usize {
        self.items.len()
    }

    fn next(&mut self) {
        self.state.select(Some(self.next_index()))
    }

    fn previous(&mut self) {
        self.state.select(Some(self.previous_index()))
    }
}

#[derive(Default)]
pub struct BoardPageState {
    pub state: ListState,
    pub items: Vec<BoardPost>,
    pub id: String,
    pub name: String,
}

impl BoardPageState {
    pub fn id(&mut self, id: String) {
        self.id = id;
    }

    pub fn name(&mut self, name: String) {
        self.name = name;
    }

    pub fn items(&mut self, items: Vec<BoardPost>) {
        self.items = items;
    }
}

impl ListStateInit<BoardPost> for BoardPageState {
    fn lists(&self) -> &Vec<BoardPost> {
        &self.items
    }

    fn state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl CursorMoveable for BoardPageState {
    fn index(&self) -> usize {
        self.state.selected().map_or(1, |x|x)
    }

    fn max(&self) -> usize {
        self.items.len()
    }

    fn next(&mut self) {
        self.state.select(Some(self.next_index()))
    }

    fn previous(&mut self) {
        self.state.select(Some(self.previous_index()))
    }
}