use reqwest::Error;
use reqwest::IntoUrl;
use reqwest::Response;

pub async fn post_request<U: IntoUrl>(url: U, data: serde_json::Value) -> Result<Response, Error> {
    // Create a client instance
    let client = reqwest::Client::new();

    // Send a POST request
    let res = client.post(url).json(&data).send().await?;

    // Optionally, handle the response e.g., check status, parse body, etc.
    if res.status().is_success() {
        println!("Successfully sent the POST request");
    } else {
        println!("Failed to send POST request: {}", res.status());
    }

    Ok(res)
}
