use bahamut::api::search::SearchResult;

pub enum FetchDataMsg {
    SearchResult(Vec<SearchResult>),
}

pub enum DataRequestMsg {
    SearchResult(String),
    End,
}
