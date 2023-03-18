use scraper::{Html, Selector, ElementRef};
use url::Url;

use super::{user::User, WebSite};

pub trait CommentReadable {
    fn comment(&self) -> Vec<PostComment>;
}

pub trait ReplyReadable {
    fn reply(&self) -> Vec<PostReply>;
}

pub type PostDescription = Vec<String>;

pub struct Post {
    pub title: String,
    pub desc: PostDescription,
    pub id: String,
    pub user: User,
    pub date: String,

    document: Html,
}

impl CommentReadable for Post {
    fn comment(&self) -> Vec<PostComment> {
        vec![]
    }
}

impl ReplyReadable for Post {
    fn reply(&self) -> Vec<PostReply> {
        let selector = Post::get_root_elm_selector();
        self.document.select(&selector)
            .skip(1)
            .filter_map(|dom| {
                let reply = PostReply {
                    id: String::from(""),
                    desc: Post::try_desc_from_html(&dom)?,
                    user: User::default(),
                    floor: PostReply::try_floor_from_html(&dom)?,
                };

                Some(reply)
            })
            .collect::<Vec<PostReply>>()
    }
}

impl Post {
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
            id: Post::try_id_from_url(&url).ok_or("post id invalid")?,
            title: Post::try_title_from_html(&top_post_elm).ok_or("post title invalid")?,
            desc: Post::try_desc_from_html(&top_post_elm).ok_or("post desc invalid")?,
            user: User::try_from(&top_post_elm)?,
            date: Post::try_date_from_html(&top_post_elm).ok_or("post date invalid")?,
            document,
        };

        Ok(post)
    }
}

impl Into<Html> for Post {
    fn into(self) -> Html {
        self.document
    }
}

pub struct PostComment {
    name: String,
    comment: String,
    id: String,
}

pub struct PostReply {
    id: String,
    desc: PostDescription,
    user: User,
    floor: u16,
}

impl CommentReadable for PostReply {
    fn comment(&self) -> Vec<PostComment> {
        vec![]
    }
}

impl PostReply {
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
}
