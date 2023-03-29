use bahamut::api::{search::SearchResult, board::BoardPost};

pub enum FetchDataMsg {
    SearchResult(Vec<SearchResult>),
    BoardPage(Vec<BoardPost>),
}

pub enum DataRequestMsg {
    SearchResult(String),
    BoardPage(String, u16),
    End,
}
