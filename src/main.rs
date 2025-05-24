use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Quote {
    q: String, // the quote
    a: String, // the author
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching an inspirational quote...");

    let url = "https://zenquotes.io/api/random";
    let response = reqwest::get(url).await?.json::<Vec<Quote>>().await?;

    if let Some(quote) = response.first() {
        println!("\"{}\" — {}", quote.q, quote.a); 
    } else {
        println!("Could not fetch a quote.");
    }

    Ok(())
}
