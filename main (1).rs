use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let model = "llama3.2:latest";
    let url = "http://localhost:11434/api/generate";

    println!("🦀 مرحبًا بك في شات Rust مع Llama 3.2!");
    loop {
        print!("أنت: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if input == "exit" {
            println!("👋 تم إغلاق الشات.");
            break;
        }

        let request_body = OllamaRequest {
            model: model.to_string(),
            prompt: input.to_string(),
            stream: false,
        };

        let response = client
            .post(url)
            .json(&request_body)
            .send()
            .await?
            .json::<OllamaResponse>()
            .await?;

        println!("🤖 Llama: {}", response.response);
    }

    Ok(())
}