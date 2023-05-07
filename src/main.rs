use api::api_types::Root;
use dotenv::dotenv;
use reqwest;
use std::env;

mod api {
    pub mod api_types;
    pub mod fetch_songs;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_url = env::var("API").unwrap();
    let title = "Kesha";

    let request_format = format!("{}{}", api_url, title);

    let response = reqwest::get(request_format).await?.json::<Root>().await?;

    println!("{:?}", response.data[0]);

    Ok(())
}
