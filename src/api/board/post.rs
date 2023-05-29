use crate::api::{UrlWithId, DN};

use super::category::{BoardCategory, BoardCategoryId};

use scraper::{ElementRef, Selector};
use url::Url;

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
                },
            },
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
                        post.floor(v.parse::<u16>().map_or(0, |v| v));
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
        let selector =
            Selector::parse(".b-list__count__number span").expect("parse selector error");
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
            let url =
                Url::parse(format!("{}/{}", DN, href).as_str()).expect("invalid category url");

            post.category(BoardCategory {
                name,
                id: BoardCategoryId::try_from(url).expect("invalid category url"),
            });
        }

        Ok(post)
    }
}
