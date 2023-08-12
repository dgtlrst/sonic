// sonic backs up spotify playlist data

use serde::Deserialize;
use tokio::main;

#[derive(Debug)]
struct Playlist {
    name: String,
    id: String,
    // Add more fields as needed
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set your Spotify access token here
    let access_token = "YOUR_ACCESS_TOKEN";

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/playlists")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if response.status().is_success() {
        let playlists = response.bytes().await?;
        println!("{:#?}", playlists);
    } else {
        println!("Error: {}", response.status().to_string());
    }

    Ok(())
}


