use scraper::{ElementRef, Selector};
use url::Url;

use crate::api::board::BoardCategoryId;

use super::{user::User, board::BoardCategory};

pub trait CommentReadable {
    fn comment(&self) -> Vec<PostReply>;
}

pub struct Post {
    title: String,
    date: String,
    user: User,
    desc: String,
    category: BoardCategory,
    gp: u16,
    reply: u16,
}

impl CommentReadable for Post {
    fn comment(&self) -> Vec<PostReply> {
        vec![]
    }
}

impl Default for Post {
    fn default() -> Self {
        const empty: &str = "";
        Post {
            title: empty.to_string(),
            date: empty.to_string(),
            desc: empty.to_string(),
            user: User::default(),
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

impl Post {
    pub async fn reply(&self) -> Vec<PostComment> {
        vec![]
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

    pub fn user(&mut self, user: User) -> &Self {
        self.user = user;
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

impl TryFrom<ElementRef<'_>> for Post {
    type Error = &'static str;

    fn try_from(elm: ElementRef) -> Result<Self, &'static str> {
        let mut post = Post::default();

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
            let str = text.as_ref();
            post.gp(u16::from_str_radix(str, 16).unwrap());
        }

        // reply
        let selector = Selector::parse(".b-list__count__number").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let text = dom.text().collect::<String>();
            let str = text.as_ref();
            post.reply_count(u16::from_str_radix(str, 16).unwrap());
        }

        // category
        let selector = Selector::parse(".b-list__summary__sort").expect("parse selector error");
        if let Some(dom) = elm.select(&selector).next() {
            let name = dom.text().collect::<String>();
            let href = dom.value().attr("href").unwrap();
            let url = Url::parse(href).expect("invalid category url");

            post.category(BoardCategory {
                name,
                id: BoardCategoryId::try_from(url).expect("invalid category url"),
            });
        }

        Ok(post)
    }
}

pub struct PostComment {
    name: String,
    comment: String,
    id: String,
}

pub struct PostReply {
    desc: String,
    user: User,
}

impl CommentReadable for PostReply {
    fn comment(&self) -> Vec<PostReply> {
        vec![]
    }
}
