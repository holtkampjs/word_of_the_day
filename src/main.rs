#![allow(unused)]
use reqwest::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct WordOfTheDay {
    #[serde(rename(deserialize = "contentProvider"))]
    content_provider: HashMap<String, Value>,
    definitions: Vec<HashMap<String, Value>>,
    examples: Vec<HashMap<String, Value>>,
    #[serde(rename(deserialize = "htmlExtra"))]
    html_extra: Option<String>,
    note: String,
    word: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("WORDNIK_API_KEY").unwrap();
    let url = "https://api.wordnik.com/v4/words.json/wordOfTheDay?api_key=".to_owned() + &api_key;
    let word_of_the_day: WordOfTheDay =
        reqwest::Client::new().get(url).send().await?.json().await?;
    println!("{:#?}", word_of_the_day);
    Ok(())
}
