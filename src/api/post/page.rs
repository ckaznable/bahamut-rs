use std::collections::HashMap;

use scraper::{Html, Selector, ElementRef};
use url::Url;

use crate::api::{CachedPage, DN};

use super::post::Post;

#[derive(Default)]
pub struct PostPageUrlParameter {
    board_id: String,
    id: String,
    floor: u16,
}

impl TryFrom<String> for PostPageUrlParameter {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let url = Url::parse(value.as_str()).map_err(|_| "invalid url string").unwrap();
        PostPageUrlParameter::try_from(url).map_err(|_| "")
    }
}

impl TryFrom<Url> for PostPageUrlParameter {
    type Error = &'static str;
    fn try_from(url: Url) -> Result<Self, Self::Error> {
        let mut ppup = PostPageUrlParameter::default();
        url.query_pairs().for_each(|(k, v)| {
            if k == "snA" {
                ppup.id = v.to_string();
            }

            if k == "bsn" {
                ppup.board_id = v.to_string();
            }

            if k == "tnum" {
                ppup.floor = v.to_string().parse::<u16>().map_or(0, |v|v);
            }
        });

        Ok(ppup)
    }
}

pub struct PostPage {
    pub board_id: String,
    pub id: String,
    pub page: u16,
    pub max: u16,
    pub floor: u16,

    cache: HashMap<u16, Option<Post>>,
    first_page_html: Option<Html>,
}

impl PostPage {
    pub fn new(board_id: &str, id: &str) -> PostPage {
        PostPage {
            board_id: board_id.to_string(),
            id: id.to_string(),
            page: 1,
            max: 0,
            floor: 0,
            cache: HashMap::new(),
            first_page_html: None,
        }
    }

    pub fn init(&mut self) {
        if let Some(document) = self.get_page_html(1) {
            let root = document.root_element();
            let max = PostPage::try_page_from_html(&root).map_or(0, |v|v);
            self.max = max;
            self.first_page_html = Some(document);
        }
    }

    pub fn floor(&mut self, floor: u16) {
        self.floor = floor;
    }

    fn try_page_from_html(document: &ElementRef) -> Option<u16> {
        let selector = Selector::parse(".BH-pagebtnA a").unwrap();
        let max: u16 = document
            .select(&selector)
            .last()?
            .text()
            .next()?
            .to_string()
            .parse()
            .unwrap();

        Some(max)
    }
}

impl CachedPage<Post> for PostPage {
    fn cache(&self) -> &HashMap<u16, Option<Post>> {
        &self.cache
    }

    fn insert_cache(&mut self, page: &u16, obj: Option<Post>) {
        self.cache.insert(*page, obj);
    }

    fn cached_page_html(&self, page: u16) -> Option<Html> {
        if page == 1 {
            self.first_page_html.clone()
        } else {
            None
        }
    }

    fn url(&self, page: &u16) -> Url {
        let url = format!("{}C.php?bsn={}&snA={}&page={}&tnum={}", DN, self.board_id, self.id, page, self.floor);
        Url::parse(url.as_ref()).unwrap()
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

impl TryFrom<PostPageUrlParameter> for PostPage {
    type Error = &'static str;

    fn try_from(value: PostPageUrlParameter) -> Result<Self, Self::Error> {
        let PostPageUrlParameter { board_id, id, floor } = value;
        let mut page = PostPage::new(board_id.as_ref(), id.as_ref());
        page.floor(floor);
        Ok(page)
    }
}

pub struct PostPageRef {
    pub board_id: String,
    pub id: String,
    pub page: u16,
    pub max: u16,
    pub floor: u16,
}

impl TryFrom<&PostPage> for PostPageRef {
    type Error = &'static str;
    fn try_from(value: &PostPage) -> Result<Self, Self::Error> {
        Ok(
            PostPageRef {
                board_id: value.board_id.to_owned(),
                id: value.id.to_owned(),
                page: value.page,
                max: value.page,
                floor: value.floor,
            }
        )
    }
}