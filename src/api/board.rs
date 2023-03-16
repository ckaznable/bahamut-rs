use std::collections::HashMap;

use scraper::{Html, Selector};
use url::Url;

use super::{DN, post::Post, WebSite};

pub async fn get_board(id: &str) -> Board {
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

pub trait BoardPost {
    fn id(&self) -> String;
    fn post(&self) -> Vec<Post>;
}

pub struct Board {
    pub id: String,
    pub name: String,
    pub category: HashMap<String, BoardCategory>,

    document: Html,
}

impl BoardPost for Board {
    fn id(&self) -> String {
        self.id.to_owned()
    }

    fn post(&self) -> Vec<Post> {
        let selector = Selector::parse(".b-list__row").expect("parse selector error");
        self.document
            .select(&selector)
            .into_iter()
            .map(|root| {
                Post::try_from(root).map_or(Post::default(), |x|x)
            })
            .collect::<Vec<Post>>()
    }
}

impl Board {
    pub fn url(id: &str) -> Url {
        let url = format!("{}{}?bsn={}", DN, "B.php", id);
        Url::parse(url.as_ref()).expect("invalid url")
    }

    fn try_name_from_html(document: &Html) -> Option<String> {
        let selector = Selector::parse("head name").expect("parse selector error");
        let title = document
            .select(&selector)
            .next()
            .expect("get name fail")
            .text()
            .collect::<String>();

        let name = title.split(" ").next().map(String::from);
        name
    }

    fn try_id_from_url(url: &Url) -> String {
        let query = url.query_pairs()
            .find(|(k, _)| k == "bsn")
            .map(|(_, v)|v)
            .unwrap();

        query.to_string()
    }

    fn try_category_map_from_html(document: &Html) -> HashMap<String, BoardCategory> {
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

        map
    }
}

impl TryFrom<WebSite> for Board {
    type Error = &'static str;

    fn try_from(web: WebSite) -> Result<Self, &'static str> {
        let WebSite { document, url } = web;

        Ok(Board {
            name: Board::try_name_from_html(&document).map_or(String::from(""), |v|v),
            id: Board::try_id_from_url(&url),
            category: Board::try_category_map_from_html(&document),
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

impl BoardCategory {
    pub fn url(id: &str, sub_id: &str) -> Url {
        let url = format!("{}{}?bsn={}&subbsn={}", DN, "B.php", sub_id, id);
        Url::parse(url.as_ref()).expect("invalid url")
    }

    pub fn id(&self) -> String {
        self.id.sub_id.to_owned()
    }

    pub fn board_id(&self) -> String {
        self.id.id.to_owned()
    }
}