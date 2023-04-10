use scraper::{Selector, ElementRef};
use serde::Serialize;

use crate::api::user::User;

use super::{PostDescription, comment::PostComment};

pub trait CommentReadable {
    fn comment(&self) -> Vec<PostComment>;
}

#[derive(Clone, Serialize, Default)]
pub struct PostContent {
    pub id: String,
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
    pub fn try_floor_from_html(document: &ElementRef) -> Option<u16> {
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

    pub fn try_id_from_html(document: &ElementRef) -> Option<String> {
        let selector = Selector::parse(".c-article").unwrap();
        let id = document.select(&selector)
            .next()?
            .value()
            .id()?;

        Some(id.replace("cf", ""))
    }

    pub fn try_desc_from_html(document: &ElementRef) -> Option<PostDescription> {
        let selector = Selector::parse(".c-article__content").unwrap();
        let desc_selector = Selector::parse("div").unwrap();

        let desc = document
            .select(&selector)
            .flat_map(|el| {
                let content = el.select(&desc_selector);
                let is_pure_text = content.clone().next().is_none();

                if is_pure_text {
                    return el.text().map(|s|s.to_string()).collect::<PostDescription>();
                }

                content.flat_map(|el| {
                    // youtube
                    let yt_selector = Selector::parse(".video-youtube iframe").unwrap();
                    let yt = el.select(&yt_selector).next();
                    if yt.is_some() {
                        return vec![yt.unwrap().value().attr("data-src").unwrap().to_string()];
                    }

                    // image
                    let img_selector = Selector::parse("a img").unwrap();
                    let img_dom = el.select(&img_selector);
                    let img = img_dom.clone().next();
                    if img.is_some() {
                        return img_dom.map(|_img| {
                                _img.value().attr("data-src").unwrap().to_string()
                            })
                            .collect::<Vec<String>>()
                    }

                    vec![el.text().collect::<String>()]
                })
                .collect::<PostDescription>()
            })
            .collect::<PostDescription>();

        Some(desc)
    }

    pub fn try_date_from_html(document: &ElementRef) -> Option<String> {
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