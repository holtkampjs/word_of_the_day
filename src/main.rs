#![allow(unused)]

struct WordOfTheDay {
    word: String,
    category: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("WORDNIK_API_KEY").unwrap();
    let url = "https://api.wordnik.com/v4/words.json/wordOfTheDay?api_key=".to_owned() + &api_key;
    let response = reqwest::blocking::get(url)?.text()?;
    let json: serde_json::Value = serde_json::from_str(&response)?;
    println!("{:#?}", json);
    Ok(())
}
