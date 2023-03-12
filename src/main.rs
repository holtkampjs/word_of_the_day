use reqwest::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Definition {
    source: String,
    text: String,
    note: Option<String>,
    #[serde(rename(deserialize = "partOfSpeech"))]
    part_of_speech: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExampleUsage {
    url: String,
    title: String,
    text: String,
    id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct WordOfTheDay {
    definitions: Vec<Definition>,
    examples: Vec<ExampleUsage>,
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
