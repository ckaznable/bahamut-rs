use bahamut::api::{search::SearchResult, board::BoardPost};
use ratatui::widgets::{ListState, TableState};

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

pub struct AppState {
    pub page: Page,
    pub search: SearchPageState,
    pub board: BoardPageState,
    pub loading: bool,
}

impl AppState {
    pub fn new() -> AppState {
        AppState::default()
    }

    pub fn get_page(&self) -> &dyn CursorMoveable {
        match self.page {
            Page::Search => &self.search,
            _ => todo!()
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            page: Page::Search,
            search: SearchPageState::default(),
            board: BoardPageState::default(),
            loading: false,
        }
    }
}

#[derive(Default, Clone)]
pub struct SearchPageState {
    pub state: TableState,
    pub items: Vec<SearchResult>,
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