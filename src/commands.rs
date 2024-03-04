use std::process;
use reqwest::Client;

use serde::Deserialize;

#[derive(Deserialize)]
struct ReadmeRes {
    name: String,
}


pub async fn get_readme(owner: &str, repository: &str) {
    let base_url = "https://api.github.com";

    let client = Client::new();

    let res = client
        .get(format!(
            "{base_url}/repos/{owner}/{repository}/readme"
        ))
        .header("User-Agent", owner)
        .send()
        .await;

    match res {
        Ok(response) => println!("{:#?}", response.json::<ReadmeRes>().await.expect("").name),
        Err(error) => {
            eprintln!("Has encountred the error: {}", error);
            process::exit(1)
        }
    }
}

// #[derive(Deserialize)]
// struct SearchedRepositories {
//     items: String,
// }

pub async fn search_repos(repository: &str, page: u16) {
    let base_url = "https://api.github.com";
    let client = Client::new();

    let request = client.get(
        format!("{base_url}/search/repositories/?q={repository}&page={page}")
    )
    .header("User-Agent", repository)
    .send().await;

    match request {
        Ok(reponsonse) => { 
            println!("{:#?}", reponsonse.headers());

            println!("{}", reponsonse.text().await.expect("Some")); 
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1)
        }
    }
}