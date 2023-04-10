use serde::Serialize;

use url::Url;

use crate::api::{UrlWithId, DN};

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
pub struct BoardCategory {
    pub name: String,
    pub id: BoardCategoryId,
}

impl UrlWithId<(&str, &str)> for BoardCategory {
    fn url(p: (&str, &str)) -> Url {
        let url = format!("{}{}?bsn={}&subbsn={}", DN, "B.php", p.0, p.1);
        Url::parse(url.as_ref()).expect("invalid url")
    }
}

impl BoardCategory {
    pub fn id(&self) -> String {
        self.id.sub_id.to_owned()
    }

    pub fn board_id(&self) -> String {
        self.id.id.to_owned()
    }
}