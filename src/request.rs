use reqwest::blocking::Client;
use std::collections::HashMap;
use std::process::exit;
use serde_json::Value;

pub fn make(
    server_url: &str,
    headers: Option<HashMap<String, String>>,
    params: Option<HashMap<String, String>>
) -> Value {
    // Create a reqwest client
    let client = Client::new();

    // Build the URL with parameters if provided
    let mut url = reqwest::Url::parse(server_url).expect("Invalid URL");
    if let Some(query_params) = params {
        let query: Vec<(String, String)> = query_params.into_iter().collect();
        url.query_pairs_mut().extend_pairs(query);
    }

    // Prepare the request builder
    let mut request = client.get(url);

    // Add headers if provided
    if let Some(custom_headers) = headers {
        for (key, value) in custom_headers {
            request = request.header(&key, value);
        }
    }

    // Make the request
    match request.send() {
        Ok(response) => {
            match response.json::<Value>() {
                Ok(json) => {
                    // Try to get the value of "detail" if it exists
                    if let Some(detail) = json.get("detail") {
                        detail.clone()
                    } else {
                        // Return null if "detail" key is not present
                        Value::Null
                    }
                },
                Err(err) => {
                    println!("Failed to parse response as JSON: {}", err);
                    exit(1);
                }
            }
        },
        Err(err) => {
            println!("Failed to fetch data from {}: {}", server_url, err);
            exit(1);
        }
    }
}
