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
            .map(|dom| {
                PostReply {
                    id: String::from(""),
                    desc: Post::try_desc_from_html(&dom),
                    user: User::default(),
                }
            })
            .collect::<Vec<PostReply>>()
    }
}

impl Post {
    fn get_root_elm_selector() -> Selector {
        Selector::parse(".c-post__body").unwrap()
    }

    fn try_id_from_url(url: &Url) -> String {
        let query = url.query_pairs()
            .find(|(k, _)| k == "snA")
            .map(|(_, v)|v)
            .unwrap();

        query.to_string()
    }

    fn try_title_from_html(document: &Html) -> String {
        let selector = Selector::parse(".c-post__header__title").unwrap();
        document.select(&selector).next().unwrap().text().collect::<String>()
    }

    fn try_desc_from_html(document: &ElementRef) -> PostDescription {
        let selector = Selector::parse(".c-article__content").unwrap();
        document.select(&selector).next().unwrap().text().map(|s|s.to_string()).collect::<PostDescription>()
    }
}

impl TryFrom<WebSite> for Post {
    type Error = &'static str;

    fn try_from(web: WebSite) -> Result<Self, Self::Error> {
        let WebSite { url, document } = web;
        let selector = Post::get_root_elm_selector();
        let top_post_elm= document.select(&selector).next().unwrap();

        let post = Post {
            id: Post::try_id_from_url(&url),
            title: Post::try_title_from_html(&document),
            desc: Post::try_desc_from_html(&top_post_elm),
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
}

impl CommentReadable for PostReply {
    fn comment(&self) -> Vec<PostComment> {
        vec![]
    }
}
