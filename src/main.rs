mod api;

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    reqwest::get("https://forum.gamer.com.tw/B.php?bsn=60030")
        .await?
        .text()
        .await?;

    Ok(())
}
