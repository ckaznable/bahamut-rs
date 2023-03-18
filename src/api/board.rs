use std::collections::HashMap;

use scraper::{Html, Selector, ElementRef};
use url::Url;

use super::{DN, WebSite, UrlWithId};

pub async fn get_board(id: &str) -> Board {
    get_board_with_page(id, 1u16).await
}

pub async fn get_board_with_page(id: &str, page: u16) -> Board {
    let url = Board::url(id);
    let html = reqwest::get(url.as_str())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(html.as_ref());
    Board::try_from(WebSite { url, document }).unwrap()
}

pub trait PostReadable {
    fn id(&self) -> String;
    fn post(&self) -> Vec<BoardPost>;
}

pub struct BoardPage {
    page: u16,
    limit: u16,
    board: Board,
}

impl BoardPage {
    fn try_page_from_html(document: &Html) -> Option<u16> {
        let selector = Selector::parse(".BH-pagebtnA a").unwrap();
        let last = document.select(&selector).last().unwrap();
        let page: u16 = last.text().collect::<String>().parse().unwrap();
        Some(page)
    }
}

impl TryFrom<WebSite> for BoardPage {
    type Error = &'static str;

    fn try_from(web: WebSite) -> Result<Self, Self::Error> {
        let page = BoardPage {
            page: 1u16,
            limit: BoardPage::try_page_from_html(&web.document).ok_or("invalid page limit")?,
            board: Board::try_from(web)?,
        };

        Ok(page)
    }
}

pub struct Board {
    pub id: String,
    pub name: String,
    pub category: HashMap<String, BoardCategory>,

    document: Html,
}

impl PostReadable for Board {
    fn id(&self) -> String {
        self.id.to_owned()
    }

    fn post(&self) -> Vec<BoardPost> {
        let selector = Selector::parse(".b-list__row").expect("parse selector error");
        self.document
            .select(&selector)
            .into_iter()
            .map(|root| {
                BoardPost::try_from(root).map_or(BoardPost::default(), |x|x)
            })
            .collect::<Vec<BoardPost>>()
    }
}

impl UrlWithId<&str> for Board {
    fn url(id: &str) -> Url {
        let url = format!("{}{}?bsn={}", DN, "B.php", id);
        Url::parse(url.as_ref()).expect("invalid url")
    }
}

impl Board {
    fn try_name_from_html(document: &Html) -> Option<String> {
        let selector = Selector::parse("head title").expect("parse selector error");
        let title = document
            .select(&selector)
            .next()
            .expect("get name fail")
            .text()
            .collect::<String>();

        let name = title.split(" ").next().map(String::from);
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
            .into_iter()
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

impl Into<Html> for Board {
    fn into(self) -> Html {
        self.document
    }
}

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
    id: String,
    title: String,
    date: String,
    desc: String,
    category: BoardCategory,
    gp: u16,
    reply: u16,
}

impl Default for BoardPost {
    fn default() -> Self {
        let empty: &str = "";
        BoardPost {
            id: String::from("0"),
            title: empty.to_string(),
            date: empty.to_string(),
            desc: empty.to_string(),
            gp: 0,
            reply: 0,
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
            Url::parse(url.as_str())
                .unwrap()
                .query_pairs()
                .into_iter()
                .for_each(|(k, v)| {
                    if k == "snA" {
                        post.id(v.to_string());
                    }
                });
        }

        // title
        let selector = Selector::parse(".b-list__tile").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            post.title(dom.text().collect::<String>());
        }

        // description
        let selector = Selector::parse(".b-list__brief").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            post.desc(dom.text().collect::<String>());
        }

        // gp
        let selector = Selector::parse(".b-list__summary__gp").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let text = dom.text().collect::<String>();
            post.gp(text.parse::<u16>().unwrap());
        }

        // reply
        let selector = Selector::parse(".b-list__count__number span").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let text = dom.text().collect::<String>();
            post.reply_count(text.parse::<u16>().unwrap());
        }

        // date
        let selector = Selector::parse(".b-list__time__edittime a").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let text = dom.text().collect::<String>();
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
