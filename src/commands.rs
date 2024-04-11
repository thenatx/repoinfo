use base64::engine::{general_purpose, Engine};
use reqwest::Client;
use std::process;
use serde::Deserialize;

pub async fn get_readme(owner: &str, repository: &str) -> Result<(), reqwest::Error> {
    let base_url = "https://api.github.com";
    let client = Client::new();

    let response = client
        .get(format!("{base_url}/repos/{owner}/{repository}/readme"))
        .header("User-Agent", owner)
        .send()
        .await?;

    let readme_response = response.json::<ReadmeRes>().await;

    match readme_response {
        Ok(readme) => {
            println!("/{}\n", readme.path);

            println!("{}", readme.content_decode().expect("Has ocurred a error decoding file").as_str());
        }
        Err(_) => {
            eprintln!("The readme not exists");
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct GithubApiError {
    message: String
}

#[derive(Debug, Deserialize)]
struct OwnerUser {
    login: String,
}

#[derive(Debug, Deserialize)]
struct RepositoryItem {
    name: String,
    description: Option<String>,
    owner: OwnerUser,
    forks: u64,
    default_branch: String,
    homepage: Option<String>,
    open_issues: u64,
    stargazers_count: u64,
}

#[derive(Debug, Deserialize)]
struct RepositoryContentItem {
    name: String,
    path: String,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Debug, Deserialize)]
struct RepoSearchResult {
    total_count: i64,
    items: Vec<RepositoryItem>,
}

#[derive(Deserialize)]
struct ReadmeRes {
    path: String,
    content: String,
}

impl ReadmeRes {
    fn content_decode(&self) -> Result<String, base64::DecodeError> {
        let readme_content = general_purpose::STANDARD.decode(self.content.replace("\n", ""))?;

        match String::from_utf8(readme_content) {
            Ok(content_decoded) => Ok(content_decoded),
            Err(err) => {
                eprintln!("A error has ocurred while try decode string to utf8");
                eprintln!("Error: {}", err);
                process::exit(0)
            }
        }
    }
}


pub mod repo;
pub mod search;
