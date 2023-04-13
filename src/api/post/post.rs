use scraper::{ElementRef, Selector};
use url::Url;

use crate::api::{user::User, WebSite};

use super::content::PostContent;

#[derive(Clone, Default)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub posts: Vec<PostContent>,
    pub floor: u16,
}

impl Post {
    pub fn posts(document: &ElementRef) -> Vec<PostContent> {
        let selector = Post::get_root_elm_selector();
        document.select(&selector)
            .filter_map(|dom| {
                Some(
                    PostContent {
                        id: PostContent::try_id_from_html(&dom)?,
                        desc: PostContent::try_desc_from_html(&dom)?,
                        user: User::try_from(&dom).ok()?,
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

    fn try_last_floor_from_url(url: &Url) -> Option<u16> {
        let query = url.query_pairs()
            .find(|(k, _)| k == "tnum")
            .map(|(_, v)|v)?;

        Some(query.parse().unwrap())
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
            floor: Post::try_last_floor_from_url(&url).ok_or("can't get last floor")?,
            title: Post::try_title_from_html(&top_post_elm).ok_or("post title invalid")?,
            posts: Post::posts(&document.root_element()),
        };

        Ok(post)
    }
}