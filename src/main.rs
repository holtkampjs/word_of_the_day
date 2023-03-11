use reqwest::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("WORDNIK_API_KEY").unwrap();
    let url = "https://api.wordnik.com/v4/words.json/randomWord?api_key=".to_owned() + &api_key;
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
