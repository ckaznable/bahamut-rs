use std::collections::HashMap;

use scraper::{Html, Selector, ElementRef};
use serde::Serialize;
use url::Url;

use super::{DN, WebSite, UrlWithId, CachedPage};

pub trait PostReadable {
    fn id(&self) -> String;
    fn post(&self) -> Vec<BoardPost>;
}

pub struct BoardPage {
    pub id: String,
    pub page: u16,
    pub max: u16,

    cache: HashMap<u16, Option<Board>>,
    first_page_cache: Option<Html>
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
            first_page_cache: None
        }
    }

    pub fn init(&mut self) {
        if let Some(document) = self.get_page_html(1) {
            let root = document.root_element();
            let max = BoardPage::try_page_from_html(&root).map_or(0, |v|v);
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

#[derive(Clone, Serialize)]
pub struct BoardCategoryId {
    pub id: String,
    pub sub_id: String
}

impl TryFrom<Url> for BoardCategoryId {
    type Error = &'static str;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        let empty = "";
        let mut id: String = empty.to_string();
        let mut sub_id: String = empty.to_string();

        url.query_pairs().for_each(|(k, v)| {
            if k == "bsn" {
                id = v.to_string();
            } else {
                sub_id = v.to_string();
            }
        });

        Ok(BoardCategoryId { id, sub_id })
    }
}

#[derive(Clone, Serialize)]
pub struct BoardCategory {
    pub name: String,
    pub id: BoardCategoryId,
}

impl UrlWithId<(&str, &str)> for BoardCategory {
    fn url(p: (&str, &str)) -> Url {
        let url = format!("{}{}?bsn={}&subbsn={}", DN, "B.php", p.0, p.1);
        Url::parse(url.as_ref()).expect("invalid url")
    }
}

impl BoardCategory {
    pub fn id(&self) -> String {
        self.id.sub_id.to_owned()
    }

    pub fn board_id(&self) -> String {
        self.id.id.to_owned()
    }
}

pub struct BoardPost {
    pub id: String,
    pub title: String,
    pub date: String,
    pub desc: String,
    pub category: BoardCategory,
    pub gp: u16,
    pub reply: u16,
    pub floor: u16,
    pub url: String,
}

impl Default for BoardPost {
    fn default() -> Self {
        let empty: &str = "";
        BoardPost {
            id: String::from("0"),
            title: empty.to_string(),
            date: empty.to_string(),
            desc: empty.to_string(),
            url: empty.to_string(),
            gp: 0,
            reply: 0,
            floor: 0,
            category: BoardCategory {
                name: empty.to_string(),
                id: BoardCategoryId {
                    id: empty.to_string(),
                    sub_id: empty.to_string(),
                }
            }
        }
    }
}

impl UrlWithId<(&str, &str)> for BoardPost {
    fn url(p: (&str, &str)) -> Url {
        let url = format!("{}{}?bsn={}&snA={}", DN, "B.php", p.0, p.1);
        Url::parse(url.as_ref()).expect("invalid url")
    }
}

impl UrlWithId<(&str, &str, u16)> for BoardPost {
    fn url(p: (&str, &str, u16)) -> Url {
        let url = format!("{}{}?bsn={}&snA={}&tnum={}", DN, "B.php", p.0, p.1, p.2);
        Url::parse(url.as_ref()).expect("invalid url")
    }
}

impl BoardPost {
    pub fn id(&mut self, id: String) -> &Self {
        self.id = id;
        self
    }

    pub fn reply_count(&mut self, reply: u16) -> &Self {
        self.reply = reply;
        self
    }

    pub fn title(&mut self, title: String) -> &Self {
        self.title = title;
        self
    }

    pub fn date(&mut self, date: String) -> &Self {
        self.date = date;
        self
    }

    pub fn gp(&mut self, gp: u16) -> &Self {
        self.gp = gp;
        self
    }

    pub fn desc(&mut self, desc: String) -> &Self {
        self.desc = desc;
        self
    }

    pub fn category(&mut self, category: BoardCategory) -> &Self {
        self.category = category;
        self
    }

    pub fn floor(&mut self, floor: u16) -> &Self {
        self.floor = floor;
        self
    }
}

impl TryFrom<ElementRef<'_>> for BoardPost {
    type Error = &'static str;

    fn try_from(elm: ElementRef) -> Result<Self, &'static str> {
        let mut post = BoardPost::default();

        // id
        let selector = Selector::parse(".b-list__main a").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let url = dom.value().attr("href").unwrap();
            let url = format!("{}/{}", DN, url);
            post.url = url.to_owned();

            Url::parse(url.as_str())
                .unwrap()
                .query_pairs()
                .into_iter()
                .for_each(|(k, v)| {
                    if k == "snA" {
                        post.id(v.to_string());
                    }

                    if k == "tnum" {
                        post.floor(v.parse::<u16>().map_or(0, |v|v));
                    }
                });
        }

        // title
        let selector = Selector::parse(".b-list__main__title").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            post.title(dom.text().collect::<String>().trim().into());
        } else {
            return Err("ad post");
        }

        // description
        let selector = Selector::parse(".b-list__brief").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            post.desc(dom.text().collect::<String>().trim().into());
        }

        // gp
        let selector = Selector::parse(".b-list__summary__gp").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let text: String = dom.text().collect::<String>().trim().into();
            post.gp(text.parse::<u16>().unwrap());
        }

        // reply
        let selector = Selector::parse(".b-list__count__number span").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let text: String = dom.text().collect::<String>().trim().into();
            post.reply_count(text.parse::<u16>().unwrap());
        }

        // date
        let selector = Selector::parse(".b-list__time__edittime a").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let text: String = dom.text().collect::<String>().trim().into();
            post.date(text);
        }

        // category
        let selector = Selector::parse(".b-list__summary__sort a").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let name = dom.text().collect::<String>();
            let href = dom.value().attr("href").unwrap();
            let url = Url::parse(format!("{}/{}", DN, href).as_str()).expect("invalid category url");

            post.category(BoardCategory {
                name,
                id: BoardCategoryId::try_from(url).expect("invalid category url"),
            });
        }

        Ok(post)
    }
}
