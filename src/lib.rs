#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! Find all books that have Rust in their name and save the title, poster URL,
//! and book URL to a books.csv file and the poster to an image with the same name as it's on the site.

use anyhow::{Result, Ok};
use reqwest::Url;
use serde::Serialize;
use voyager::{scraper::Selector, Scraper};

/// Describes selectors for each part of the shop item.
#[derive(Debug)]
pub struct BookScraper {
  item_selector: Selector,
  image_selector: Selector,
  title_selector: Selector,
  pagination_selector: Selector,
  base_url: Url
}

impl Default for BookScraper {
  fn default() -> Self {
    Self {
      item_selector: Selector::parse(".category-title-container a").unwrap(),
      title_selector: Selector::parse(".category-title-title").unwrap(),
      image_selector: Selector::parse(".bookimage").unwrap(),
      pagination_selector: Selector::parse(".pagination-list li a").unwrap(),
      base_url: Url::parse("https://pragprog.com").unwrap()
    }
  }
}

/// Serializable shop item.
#[derive(Debug, Serialize)]
pub struct ShopItem {
  /// Book title.
  pub title: String,
  /// Book's image url.
  pub image_url: String,
  /// Link to the book in the shop.
  pub link: String,
}

/// State of the scrapper.
#[derive(Debug)]
pub enum StoreState {
  /// Scrape a book.
  Book(ShopItem),
  /// Change page.
  Page(usize)
}

impl Scraper for BookScraper {
  type Output = ShopItem;
  type State = StoreState;

  fn scrape(
    &mut self,
    response: voyager::Response<Self::State>,
    crawler: &mut voyager::Crawler<Self>,
  ) -> Result<Option<Self::Output>> {
    let html = response.html();

    if let Some(state) = response.state {
      match state {
        StoreState::Page(page) => {
          for item in html.select(&self.item_selector) {
            let title: String = item.select(&self.title_selector).next()
              .map(|d| d.text().collect()).unwrap();
            let image = item.select(&self.image_selector).next()
              .map(|d| d.value().attr("src").unwrap()).unwrap();
            let link = item.value().attr("href").unwrap().to_owned();
            let shop_item = ShopItem {
              title,
              image_url: format!("{}{image}", self.base_url),
              link
            };
            crawler.visit_with_state(shop_item.link.clone(), StoreState::Book(shop_item))
          }
          if let Some(next_page_url) = html.select(&self.pagination_selector)
            .last().unwrap().value().attr("href") {
                dbg!(&next_page_url);
              crawler.visit_with_state(format!("{}{next_page_url}", self.base_url), StoreState::Page(page + 1))
            }
        },
        StoreState::Book(book) => {
          return Ok(Some(book));
        }
      }
    } else {
      crawler.visit_with_state(response.response_url.to_string(), StoreState::Page(0))
    }
    Ok(None)
  }
}