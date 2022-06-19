use std::error::Error;

use serde_json::Value;

pub mod response;

pub async fn request(data: &str, port: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("http://localhost:{}", port);

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .body(data.to_string())
        .send()
        .await
        .map_err(|error| format!("{}", error))?;

    let result = response.text().await?;

    Ok(result)
}