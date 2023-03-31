use std::collections::HashMap;

use scraper::{Selector, ElementRef};
use serde::Serialize;
use url::Url;

use super::{user::User, WebSite, CachedPage, DN};

pub trait CommentReadable {
    fn comment(&self) -> Vec<PostComment>;
}

pub type PostDescription = Vec<String>;

pub struct PostPage {
    pub board_id: String,
    pub id: String,
    pub page: u16,
    pub max: u16,

    cache: HashMap<u16, Option<Post>>,
}

impl PostPage {
    pub fn new(board_id: &str, id: &str) -> PostPage {
        PostPage {
            board_id: board_id.to_string(),
            id: id.to_string(),
            page: 1,
            max: 0,
            cache: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        let document = self.get_page_html(1);
        let root = document.root_element();
        let max = PostPage::try_page_from_html(&root).map_or(0, |v|v);
        self.max = max;
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

    fn url(&self, page: &u16) -> Url {
        let url = format!("{}C.php?bsn={}&snA={}&page={}", DN, self.board_id, self.id, page);
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

#[derive(Clone, Default)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub posts: Vec<PostContent>,
}

impl Post {
    pub fn posts(document: &ElementRef) -> Vec<PostContent> {
        let selector = Post::get_root_elm_selector();
        document.select(&selector)
            .filter_map(|dom| {
                Some(
                    PostContent {
                        desc: PostContent::try_desc_from_html(&dom)?,
                        user: User::try_from(&dom).map_or(None, |x|Some(x))?,
                        floor: PostContent::try_floor_from_html(&dom)?,
                        date: PostContent::try_date_from_html(&dom)?,
                    }
                )
            })
            .collect::<Vec<PostContent>>()
    }

    fn get_root_elm_selector() -> Selector {
        Selector::parse(".c-section[id]").unwrap()
    }

    fn try_id_from_url(url: &Url) -> Option<String> {
        let query = url.query_pairs()
            .find(|(k, _)| k == "snA")
            .map(|(_, v)|v)?;

        Some(query.to_string())
    }

    fn try_title_from_html(document: &ElementRef) -> Option<String> {
        let selector = Selector::parse(".c-post__header__title").unwrap();
        let title = document
            .select(&selector)
            .next()?
            .text()
            .collect::<String>();

        Some(title)
    }

}

impl TryFrom<WebSite> for Post {
    type Error = &'static str;

    fn try_from(web: WebSite) -> Result<Self, Self::Error> {
        let WebSite { url, document } = web;
        let selector = Post::get_root_elm_selector();
        let top_post_elm= document
            .select(&selector)
            .next()
            .unwrap();

        let post = Post {
            id: Post::try_id_from_url(&url).ok_or("can't get id")?,
            title: Post::try_title_from_html(&top_post_elm).ok_or("post title invalid")?,
            posts: Post::posts(&document.root_element()),
        };

        Ok(post)
    }
}

#[derive(Clone, Serialize)]
pub struct PostComment {
    pub name: String,
    pub comment: String,
    pub id: String,
}

#[derive(Clone, Serialize)]
pub struct PostContent {
    pub desc: PostDescription,
    pub user: User,
    pub floor: u16,
    pub date: String,
}

impl CommentReadable for PostContent {
    fn comment(&self) -> Vec<PostComment> {
        vec![]
    }
}

impl PostContent {
    fn try_floor_from_html(document: &ElementRef) -> Option<u16> {
        let selector = Selector::parse(".floor").unwrap();
        let floor = document
            .select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("data-floor")
            .unwrap()
            .parse::<u16>()
            .map_or(0u16, |v|v);

        Some(floor)
    }

    fn try_desc_from_html(document: &ElementRef) -> Option<PostDescription> {
        let selector = Selector::parse(".c-article__content").unwrap();
        let desc = document
            .select(&selector)
            .next()?
            .text()
            .map(|s|s.to_string())
            .collect::<PostDescription>();

        Some(desc)
    }

    fn try_date_from_html(document: &ElementRef) -> Option<String> {
        let selector = Selector::parse(".edittime").unwrap();
        let date = document
            .select(&selector)
            .next()?
            .text()
            .next()?
            .to_string();

        Some(date)
    }
}
