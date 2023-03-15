use reqwest::Result;
use word_of_the_day::WordOfTheDay;

pub mod word_of_the_day;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("WORDNIK_API_KEY").unwrap();
    let url = "https://api.wordnik.com/v4/words.json/wordOfTheDay?api_key=".to_owned() + &api_key;
    let word_of_the_day: WordOfTheDay =
        reqwest::Client::new().get(url).send().await?.json().await?;

    word_of_the_day.display();

    Ok(())
}
