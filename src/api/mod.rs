use lazy_static::lazy_static;

use std::{collections::HashMap, time::Duration};

use futures::executor::block_on;
use scraper::Html;
use serde::de::DeserializeOwned;
use url::Url;

pub mod board;
pub mod post;
pub mod search;
pub mod user;

pub static DN: &str = "https://forum.gamer.com.tw/";

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
}

async fn get_document(url: &Url) -> Result<Html, Box<dyn std::error::Error>> {
    let html = HTTP_CLIENT.get(url.as_str()).send().await?.text().await?;

    Ok(Html::parse_document(html.as_ref()))
}

async fn get_json<T: DeserializeOwned>(url: &Url) -> Result<T, Box<dyn std::error::Error>> {
    let res = HTTP_CLIENT
        .get(url.as_str())
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(res)
}

pub struct WebSite {
    pub url: Url,
    pub document: Html,
}

pub trait UrlWithId<T> {
    fn url(p: T) -> Url;
}

pub trait CachedPage<T>
where
    T: Sized + TryFrom<WebSite> + Clone,
{
    fn cache(&self) -> &HashMap<u16, Option<T>>;
    fn insert_cache(&mut self, page: &u16, obj: Option<T>);
    fn url(&self, page: &u16) -> Url;
    fn page(&self) -> u16;
    fn increase_page(&mut self);
    fn decrease_page(&mut self);
    fn max(&self) -> u16;

    fn cached_page_html(&self, _: u16) -> Option<Html> {
        None
    }

    fn is_over_min(&self) -> bool {
        self.page() == 0
    }

    fn is_over_max(&self) -> bool {
        let max = self.max();
        max != 0 && self.page() > max
    }

    fn get_page_html(&self, page: u16) -> Option<Html> {
        let url = self.url(&page);
        block_on(get_document(&url)).ok()
    }

    fn get(&self, page: u16, ignore_cache: bool) -> Option<T> {
        let max = self.max();
        if max != 0 && page > max {
            return None;
        }

        let cache = self.cache();
        if !ignore_cache && cache.contains_key(&page) {
            return cache.get(&page).unwrap().as_ref().cloned();
        }

        let document = if let Some(v) = self.cached_page_html(page) {
            Some(v)
        } else {
            self.get_page_html(page)
        };

        if let Some(document) = document {
            let url = self.url(&page);
            if let Ok(board) = T::try_from(WebSite { url, document }) {
                return Some(board);
            }
        }

        None
    }

    fn get_and_cache(&mut self, page: u16, ignore_cache: bool) -> Option<T> {
        let result = self.get(page, ignore_cache);
        self.insert_cache(&page, result.clone());
        result
    }

    fn get_current(&self) -> Option<T> {
        self.get(self.page(), false)
    }

    fn get_current_and_cache(&mut self) -> Option<T> {
        self.get_and_cache(self.page(), false)
    }

    fn get_current_force(&self) -> Option<T> {
        self.get(self.page(), true)
    }

    fn get_current_force_and_cache(&mut self) -> Option<T> {
        self.get_and_cache(self.page(), true)
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
