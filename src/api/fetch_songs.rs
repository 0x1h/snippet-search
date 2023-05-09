use dotenv::dotenv;
use reqwest::Error;
use std::env;
use crate::api::api_types::{Root, Daum};

# [allow(dead_code)]
pub async fn fetch_songs (title: String) -> Result<Vec<Daum>, Error> {
  dotenv().ok();
 
  let api_url = env::var("API").unwrap();
  let api_format = format!("{}{}", api_url, title);

  let response = reqwest::get(api_format).await?.json::<Root>().await?;
 
  return Ok(response.data);
}