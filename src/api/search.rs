use futures::executor::block_on;
use scraper::{ElementRef, Selector};
use serde::Serialize;

use crate::api::get_document;

use super::{UrlWithId, DN};

#[derive(Clone, Default, Serialize)]
pub struct BoardSearch;
impl UrlWithId<&str> for BoardSearch {
    fn url(query: &str) -> url::Url {
        url::Url::parse(format!("{}searchb.php?qt=board&search={}", DN, query).as_ref()).unwrap()
    }
}

impl BoardSearch {
    pub fn get_search_result(query: &str) -> Vec<SearchResult> {
        let url = BoardSearch::url(query);
        let document = block_on(get_document(&url));
        let selector = Selector::parse(".BH-table tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        document
            .select(&selector)
            .into_iter()
            .filter_map(|dom| {
                let td = dom.select(&td_selector).skip(2).next()?;
                SearchResult::try_from(td).map_or(None, |x|Some(x))
            })
            .collect::<Vec<SearchResult>>()
    }
}

#[derive(Clone, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub name: String,
    pub platform: String,
}

impl SearchResult {
    fn try_id_from_html(document: &ElementRef) -> Option<String> {
        let selector = Selector::parse("a").unwrap();
        let id = document
            .select(&selector)
            .next()?
            .value()
            .attr("href")?
            .split("=")
            .last()?
            .to_string();

        Some(id)
    }

    fn try_name_from_html(document: &ElementRef) -> Option<String> {
        let selector = Selector::parse("a").unwrap();
        let name = document
            .select(&selector)
            .next()?
            .text()
            .skip(1)
            .collect::<String>();

        Some(name)
    }

    fn try_platform_from_html(document: &ElementRef) -> Option<String> {
        let selector = Selector::parse("a").unwrap();
        let platform = document
            .select(&selector)
            .next()?
            .text()
            .next()?
            .to_string();

        Some(platform)
    }
}

impl TryFrom<ElementRef<'_>> for SearchResult {
    type Error = &'static str;

    fn try_from(document: ElementRef) -> Result<Self, Self::Error> {
        let id = if let Some(id) = SearchResult::try_id_from_html(&document) { id } else {
            return Err("")
        };

        let sr = SearchResult {
            id,
            name: SearchResult::try_name_from_html(&document).unwrap(),
            platform: SearchResult::try_platform_from_html(&document).unwrap(),
        };

        Ok(sr)
    }
}

