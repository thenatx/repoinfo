use reqwest::Client;
use std::process;

use serde::Deserialize;

#[derive(Deserialize)]
struct ReadmeRes {
    name: String,
}

pub async fn get_readme(owner: &str, repository: &str) {
    let base_url = "https://api.github.com";

    let client = Client::new();

    let res = client
        .get(format!("{base_url}/repos/{owner}/{repository}/readme"))
        .header("User-Agent", owner)
        .send()
        .await;

    match res {
        Ok(response) => {
            println!("{:#?}", response.json::<ReadmeRes>().await.expect("").name);
        }
        Err(error) => {
            eprintln!("Has encountred the error: {}", error);
            process::exit(1)
        }
    }
}

pub mod search;
