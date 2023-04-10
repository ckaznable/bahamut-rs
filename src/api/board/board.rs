use std::collections::HashMap;

use scraper::{Html, Selector};
use url::Url;

use crate::api::{WebSite, UrlWithId, DN};

use super::{category::{BoardCategory, BoardCategoryId}, post::BoardPost};

pub struct Board {
    pub id: String,
    pub name: String,
    pub category: HashMap<String, BoardCategory>,

    document: Html,
}

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut new_map: HashMap<String, BoardCategory> = HashMap::new();
        for (key, value) in &self.category {
            new_map.insert(key.clone(), value.clone());
        }

        Board {
            id: self.id.to_owned(),
            name: self.name.to_owned(),
            category: new_map,
            document: self.document.clone(),
        }
    }
}

impl UrlWithId<(&str, u16)> for Board {
    fn url(p: (&str, u16)) -> Url {
        let url = format!("{}{}?bsn={}&page={}", DN, "B.php", p.0, p.1);
        Url::parse(url.as_ref()).expect("invalid url")
    }
}

impl Board {
    pub fn post(&self) -> Vec<BoardPost> {
        let selector = Selector::parse(".b-list__row").expect("parse selector error");
        self.document
            .select(&selector)
            .filter_map(|root| {
                BoardPost::try_from(root).ok()
            })
            .collect::<Vec<BoardPost>>()
    }

    fn try_name_from_html(document: &Html) -> Option<String> {
        let selector = Selector::parse("head title").expect("parse selector error");
        let title = document
            .select(&selector)
            .next()
            .expect("get name fail")
            .text()
            .collect::<String>();

        let name = title.split(' ').next().map(String::from);
        name
    }

    fn try_id_from_url(url: &Url) -> Option<String> {
        let query = url.query_pairs()
            .find(|(k, _)| k == "bsn")
            .map(|(_, v)|v)
            .unwrap();

        Some(query.to_string())
    }

    fn try_category_map_from_html(document: &Html) -> Option<HashMap<String, BoardCategory>> {
        let mut map: HashMap<String, BoardCategory> = HashMap::new();

        let selector = Selector::parse(".b-tags__item a").expect("parse selector error");
        document
            .select(&selector)
            .for_each(|elm| {
                let href = elm.value().attr("href").unwrap();
                let url = Url::parse(href).unwrap();
                let id =  BoardCategoryId::try_from(url).unwrap();
                let name = elm.text().collect::<String>();

                map.insert(id.sub_id.to_owned(), BoardCategory { id, name });
            });

        Some(map)
    }
}

impl TryFrom<WebSite> for Board {
    type Error = &'static str;

    fn try_from(web: WebSite) -> Result<Self, &'static str> {
        let WebSite { document, url } = web;

        Ok(Board {
            name: Board::try_name_from_html(&document).map_or(String::from(""), |v|v),
            id: Board::try_id_from_url(&url).ok_or("id invalid")?,
            category: Board::try_category_map_from_html(&document).ok_or("category invalid")?,
            document
        })
    }
}