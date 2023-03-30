use bahamut::api::{search::SearchResult, board::BoardPost};

pub struct PageData<T> {
    pub page: u16,
    pub max: u16,
    pub items: Vec<T>,
}

pub enum FetchDataMsg {
    SearchResult(Vec<SearchResult>),
    BoardPage(PageData<BoardPost>),
}

pub enum DataRequestMsg {
    SearchResult(String),
    BoardPage(String, u16),
    End,
}
