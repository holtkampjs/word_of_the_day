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
pub(crate) struct WordOfTheDay {
    definitions: Vec<Definition>,
    examples: Vec<ExampleUsage>,
    note: String,
    word: String,
}

impl WordOfTheDay {
    pub fn display(&self) {
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
