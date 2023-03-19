use bahamut_rs::api::{board::BoardPage, CachedPage};

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let mut board_page = BoardPage::new("48053");
    println!("{}", board_page.next().unwrap().post().len());

    Ok(())
}
