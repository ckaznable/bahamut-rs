use std::collections::HashMap;

use futures::executor::block_on;
use serde::{Deserialize, Serialize};

use serde_json::Value;
use url::Url;

use crate::api::{get_json, DN};

#[derive(Clone, Serialize, Deserialize)]
pub struct PostComment {
    pub bsn: String,
    pub sn: String,
    pub userid: String,
    pub comment: String,
    pub gp: String,
    pub bp: String,
    pub wtime: String,
    pub mtime: String,
    pub state: String,
    pub floor: u16,
    pub content: String,
    pub time: String,
    pub nick: String,

    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

impl PostComment {
    pub fn get_comment(
        id: String,
        c_id: String,
    ) -> Result<Vec<PostComment>, Box<dyn std::error::Error>> {
        let url = format!("{}ajax/moreCommend.php?bsn={}&snB={}", DN, id, c_id);
        let url = Url::parse(url.as_ref()).unwrap();
        let map = block_on(get_json::<HashMap<String, Value>>(&url))?;

        let mut list = map
            .iter()
            .filter_map(|(k, v)| {
                if k == "next_snC" {
                    None
                } else {
                    let comment: PostComment = serde_json::from_value(v.clone()).unwrap();
                    Some(comment)
                }
            })
            .collect::<Vec<PostComment>>();

        list.sort_by_key(|v| v.floor);
        Ok(list)
    }
}
