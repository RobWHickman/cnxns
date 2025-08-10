use reqwest::blocking::Client;
use serde_json::Value;

// Generates a key for the FBref API
// https://fbrapi.com/documentation
// only run in main if no `API_KEY` is found in .env file
// Add to Cargo.toml: reqwest = { version = "0.11", features = ["json", "blocking"] }
pub fn generate_api_key() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.post("https://fbrapi.com/generate_api_key").send()?;

    let json: Value = response.json()?;
    let api_key = json["api_key"].as_str().unwrap().to_string();
    println!("API Key: {}", api_key);
    Ok(api_key)
}
