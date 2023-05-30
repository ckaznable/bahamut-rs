use bahamut::api::{
    board::BoardPost,
    post::{Post, PostComment},
    search::SearchResult,
};

pub struct PageData<T> {
    pub page: u16,
    pub max: u16,
    pub items: T,
}

pub enum FetchDataMsg {
    SearchResult(Vec<SearchResult>),
    BoardPage(PageData<Vec<BoardPost>>),
    PostPage(PageData<Post>),
    CommentPage(Vec<PostComment>),
}

pub enum DataRequestMsg {
    SearchResult(String),
    BoardPage(String, u16, bool),
    PostPage(String, u16, bool),
    CommentPage(String, String),
    End,
}
