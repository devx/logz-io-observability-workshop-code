use std::env;
use reqwest::header::{HeaderMap, HeaderValue};
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let matches = App::new("Marker Creator")
        .arg(
            Arg::with_name("payload")
                .short("p")
                .long("payload")
                .value_name("PAYLOAD")
                .help("Sets the JSON payload or a path to a JSON file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("api_token")
                .short("t")
                .long("api-token")
                .value_name("LOGZ_IO_API_TOKEN")
                .help("Sets the API token")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    // Retrieve the API token from command-line argument or environment variable
    let api_token = matches
        .value_of("api_token")
        .map(|t| t.to_owned())
        .or_else(|| env::var("LOGZ_IO_API_TOKEN").ok())
        .expect("API token not provided");

    // Get the JSON payload from the command-line argument or file
    let json_payload = matches.value_of("payload").unwrap();
    let payload_content = if json_payload.ends_with(".json") {
        std::fs::read_to_string(json_payload)?
    } else {
        json_payload.to_owned()
    };

    // Create the headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("X-API-TOKEN", HeaderValue::from_str(&api_token)?);

    // Send the POST request
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.logz.io/v2/markers/create-markers")
        .headers(headers)
        .body(payload_content)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        println!("Markers created successfully");
    } else {
        println!("Failed to create markers: {}", response.text().await?);
    }

    Ok(())
}
