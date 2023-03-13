use reqwest::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Definition {
    #[serde(rename(deserialize = "partOfSpeech"))]
    part_of_speech: String,
    text: String,
}

#[derive(Deserialize)]
struct ExampleUsage {
    text: String,
}

#[derive(Deserialize)]
struct WordOfTheDay {
    definitions: Vec<Definition>,
    examples: Vec<ExampleUsage>,
    note: String,
    word: String,
}

impl WordOfTheDay {
    fn display(&self) {
        println!("Word of the Day:");
        println!("  {}\n", self.word);

        println!("Definitions:");
        for definition in self.definitions.iter() {
            println!("  Part of speech: {}", definition.part_of_speech);
            println!("  Definition:");
            println!("    {}\n", definition.text);
        }

        println!("Examples:");
        for example in self.examples.iter() {
            println!("  Example:");
            println!("    {}\n", example.text);
        }

        println!("Note:");
        println!("  {}", self.note);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("WORDNIK_API_KEY").unwrap();
    let url = "https://api.wordnik.com/v4/words.json/wordOfTheDay?api_key=".to_owned() + &api_key;
    let word_of_the_day: WordOfTheDay =
        reqwest::Client::new().get(url).send().await?.json().await?;

    word_of_the_day.display();

    Ok(())
}
