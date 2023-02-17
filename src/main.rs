use anyhow::Result;
use futures::StreamExt;
use scrape_shop::BookScraper;
use tokio::{fs::File, io::AsyncWriteExt};
use voyager::{CrawlerConfig, Collector};

#[tokio::main]
async fn main() -> Result<()> {
  let conf = CrawlerConfig::default().allow_domain("pragprog.com");
  let mut collector = Collector::new(BookScraper::default(), conf);
  collector.crawler_mut().visit("https://pragprog.com/titles/");
  let mut writer = csv::Writer::from_path("books.csv")?;
  while let Some(data) = collector.next().await {
    if let Ok(item) = data {
      println!(
        "Book:\nTitle: {}\nImage: {}\nLink: {}",
        &item.title,
        &item.image_url,
        &item.link
      );
      if item.title.contains("Rust") {
        let mut file = File::create(
          format!("./{}", &item.image_url.split('/').last().unwrap())
        ).await?;
        let buf = reqwest::get(&item.image_url).await?.bytes().await?;
        file.write_all(&buf[..]).await?;
        file.flush().await?;
        file.shutdown().await?;
        writer.serialize(item)?;
      }
    }
  }
  writer.flush()?;
  Ok(())
}
