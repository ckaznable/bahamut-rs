use std::collections::HashMap;

use scraper::{ElementRef, Html, Selector};
use url::Url;

use crate::api::{CachedPage, UrlWithId};

use super::board::Board;

pub struct BoardPage {
    pub id: String,
    pub page: u16,
    pub max: u16,

    cache: HashMap<u16, Option<Board>>,
    first_page_cache: Option<Html>,
}

impl BoardPage {
    pub fn new(id: &str) -> BoardPage {
        BoardPage {
            id: id.to_string(),
            page: 1,
            max: 0,
            cache: HashMap::new(),
            first_page_cache: None,
        }
    }

    pub fn from_page(id: &str, page: u16) -> BoardPage {
        BoardPage {
            id: id.to_string(),
            page,
            max: 0,
            cache: HashMap::new(),
            first_page_cache: None,
        }
    }

    pub fn init(&mut self) {
        if let Some(document) = self.get_page_html(1) {
            let root = document.root_element();
            let max = BoardPage::try_page_from_html(&root).map_or(0, |v| v);
            self.max = max;
            self.first_page_cache = Some(document);
        }
    }

    fn try_page_from_html(document: &ElementRef) -> Option<u16> {
        let selector = Selector::parse(".BH-pagebtnA a").unwrap();
        let last = document.select(&selector).last().unwrap();
        let page: u16 = last.text().collect::<String>().parse().unwrap();
        Some(page)
    }
}

impl CachedPage<Board> for BoardPage {
    fn cache(&self) -> &HashMap<u16, Option<Board>> {
        &self.cache
    }

    fn insert_cache(&mut self, page: &u16, obj: Option<Board>) {
        self.cache.insert(*page, obj);
    }

    fn cached_page_html(&self, page: u16) -> Option<Html> {
        if page == 1 {
            self.first_page_cache.clone()
        } else {
            None
        }
    }

    fn url(&self, page: &u16) -> Url {
        Board::url((self.id.as_str(), *page))
    }

    fn page(&self) -> u16 {
        self.page
    }

    fn increase_page(&mut self) {
        self.page += 1;
    }

    fn decrease_page(&mut self) {
        self.page -= 1;
    }

    fn max(&self) -> u16 {
        self.max
    }
}
