use api::{post::Post, WebSite};
use scraper::Html;

use crate::api::post::ReplyReadable;

mod api;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let url = url::Url::parse("https://forum.gamer.com.tw/C.php?bsn=60030&snA=620925&tnum=22").unwrap();
    let text = reqwest::get(url.clone())
        .await?
        .text()
        .await?;

    let document = Html::parse_document(text.as_ref());
    let post = Post::try_from(WebSite { url, document }).unwrap();
    println!("{}", post.id);
    println!("{}", post.reply().len());

    Ok(())
}
