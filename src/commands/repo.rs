use reqwest::Client;
use std::process;

use crate::commands::RepositoryContentItem;

pub async fn repo_information(owner: &str, name: &str, show_files: bool) {
    let client = Client::new();
    let request_url = format!("https://api.github.com/repos/{owner}/{name}");

    let resquest = client
        .get(request_url)
        .header("User-Agent", owner)
        .send()
        .await;

    match resquest {
        Ok(response) => {
            let json_response = response.json::<super::RepositoryItem>().await.unwrap();

            println!("Name: {}", json_response.name);
            if let Some(description) = json_response.description {
                println!("Description: {}", description)
            } else {
            }
            println!("Owner: {}", json_response.owner.login);

            if let Some(homepage) = json_response.homepage {
                if homepage.len() > 0 {
                    println!("Homepage: {}", homepage);
                } else {
                    println!("Homepage: None");
                }
            } else {
                println!("Homepage: None");
            }

            let issues = json_response.open_issues;
            let forks = json_response.forks;
            let stars = json_response.stargazers_count;

            println!(
                "{}",
                format!(" {:<10}  {:<10}  {:<10}", stars, forks, issues)
            );

            if show_files == true {
                repo_files(owner, name).await;
            }
        }
        Err(err) => {
            eprintln!("Has ocurred a error while getting repository information");
            eprintln!("Error: {}", err);
            process::exit(1)
        }
    }
}

pub async fn repo_files(owner: &str, name: &str) {
    println!("\nRepository files:");
    let content_url = format!("https://api.github.com/repos/{owner}/{name}/contents");

    let client = Client::new();

    let request = client
        .get(content_url)
        .header("User-Agent", name)
        .send()
        .await;

    match request {
        Ok(response) => match response.json::<Vec<RepositoryContentItem>>().await {
            Ok(json_response) => {
                for item in json_response {
                    println!("\n{:-<100}", "");

                    println!("Filename: {}", item.name);
                    println!("Path: {}", item.path);
                    if item.r#type == "dir" {
                        println!("Type: Directory");
                    } else if item.r#type == "file" {
                        println!("Type: File");
                    } else {
                        println!("Type: {}", item.r#type)
                    }
                }
            }
            Err(err) => {
                eprintln!("Error on the JSON: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Bad request, error: {}", err);
            process::exit(1)
        }
    }
}
