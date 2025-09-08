use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

async fn search_web(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let client = Client::new();
    let search_url = format!("https://html.duckduckgo.com/html/?q={}", urlencoding::encode(query));
    let res = client.get(&search_url).send().await?.text().await?;
    let document = Html::parse_document(&res);
    let selector = Selector::parse(".result__snippet").unwrap();

    let snippets: Vec<String> = document
        .select(&selector)
        .take(3)
        .map(|el| el.text().collect::<String>())
        .collect();

    Ok(snippets)
}

async fn summarize_with_ollama(context: &str, question: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let prompt = format!("Based on the following search results:\n{}\nAnswer this question: {}", context, question);

    let request_body = OllamaRequest {
        model: "llama3.2:latest".to_string(),
        prompt,
        stream: false,
    };

    let res = client
        .post("http://localhost:11434/api/generate")
        .json(&request_body)
        .send()
        .await?
        .json::<OllamaResponse>()
        .await?;

    Ok(res.response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🔍 أدخل سؤالك للبحث:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let question = input.trim();

    println!("⏳ يتم البحث...");
    let snippets = search_web(question).await?;
    let context = snippets.join("\n");

    println!("⏳ يتم تلخيص النتائج...");
    let answer = summarize_with_ollama(&context, question).await?;

    println!("🤖 النتيجة:\n{}", answer);
    Ok(())
}