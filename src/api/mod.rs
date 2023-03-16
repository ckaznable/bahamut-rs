use scraper::Html;
use url::Url;

pub mod post;
pub mod board;
pub mod user;

pub static DN: &'static str = "https://forum.gamer.com.tw/";

pub struct WebSite {
    url: Url,
    document: Html
}

pub trait AwaitedPage<T> {
    fn page(&self) -> usize;
    fn next(&self) -> Option<T>;
    fn previous(&self) -> Option<T>;
    fn get(&self, page: usize) -> Option<T>;
    fn get_range(&self, start: usize, end: usize) -> Option<T>;
}
