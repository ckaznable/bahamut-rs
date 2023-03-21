use std::{collections::HashMap};

use futures::executor::block_on;
use scraper::{Html, ElementRef};
use url::Url;

pub mod post;
pub mod board;
pub mod user;
pub mod search;

pub static DN: &'static str = "https://forum.gamer.com.tw/";

async fn get_document(url: &Url) -> Html {
    let html = reqwest::get(url.as_str())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    Html::parse_document(html.as_ref())
}

pub struct WebSite {
    pub url: Url,
    pub document: Html
}

pub trait UrlWithId<T> {
    fn url(p: T) -> Url;
}

pub trait CachedPage<T> where T: Sized + TryFrom<WebSite> + Clone {
    fn cache(&self) -> &HashMap<u16, Option<T>>;
    fn insert_cache(&mut self, page: &u16, obj: Option<T>);
    fn url(&self, page: &u16) -> Url;
    fn page(&self) -> u16;
    fn increase_page(&mut self);
    fn decrease_page(&mut self);
    fn max(&self) -> u16;
    fn set_max(&mut self, max: &u16);
    fn max_from_page(document: &ElementRef) -> u16;

    fn is_over_min(&self) -> bool {
        self.page() == 0
    }

    fn is_over_max(&self) -> bool {
        let max = self.max();
        max != 0 && self.page() > max
    }

    fn get(&mut self, page: u16, ignore_cache: bool) -> Option<T> {
        let max = self.max();
        if max != 0 && page > max {
            return None;
        }

        let cache = self.cache();
        if !ignore_cache && cache.contains_key(&page) {
            return match cache.get(&page).unwrap() {
                None => None,
                Some(v) => Some(v.clone())
            };
        }

        let url = self.url(&page);
        let document = block_on(get_document(&url));
        let document_ref = &document.root_element();

        if max == 0 {
            let max = Self::max_from_page(&document_ref);
            self.set_max(&max);
        }

        let result = if let Ok(board) = T::try_from(WebSite { url, document }) {
            Some(board)
        } else {
            None
        };

        self.insert_cache(&page, result.clone());
        result
    }

    fn next(&mut self) -> Option<T> {
        if self.is_over_max() {
            None
        } else {
            self.increase_page();
            self.get(self.page(), false)
        }
    }

    fn force_next(&mut self) -> Option<T> {
        if self.is_over_max() {
            None
        } else {
            self.increase_page();
            self.get(self.page(), true)
        }
    }

    fn previous(&mut self) -> Option<T> {
        if self.is_over_min() {
            None
        } else {
            self.decrease_page();
            self.get(self.page(), false)
        }
    }

    fn force_previous(&mut self) -> Option<T> {
        if self.is_over_min() {
            None
        } else {
            self.decrease_page();
            self.get(self.page(), true)
        }
    }
}