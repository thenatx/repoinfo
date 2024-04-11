use reqwest::Client;
use std::process;

use crate::commands::RepositoryContentItem;

pub async fn repo_information(
    owner: &str,
    name: &str,
    show_files: bool,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let request_url = format!("https://api.github.com/repos/{owner}/{name}");

    let response = client
        .get(&request_url)
        .header("User-Agent", owner)
        .send()
        .await?;

    let repository = match response.json::<super::RepositoryItem>().await {
        Ok(repo) => repo,
        Err(_) => {
            let response = client
                .get(&request_url)
                .header("User-Agent", owner)
                .send()
                .await?;

            let error = response.json::<super::GithubApiError>().await.unwrap();
            println!("Error: {}", &error.message);
            process::exit(0)
        }
    };

    println!("Name: {}", repository.name);
    if let Some(description) = repository.description {
        println!("Description: {}", description)
    } else {
    }
    println!("Owner: {}", repository.owner.login);

    if let Some(homepage) = repository.homepage {
        if homepage.len() > 0 {
            println!("Homepage: {}", homepage);
        } else {
            println!("Homepage: None");
        }
    } else {
        println!("Homepage: None");
    }

    let issues = repository.open_issues;
    let forks = repository.forks;
    let stars = repository.stargazers_count;

    println!(
        "{}",
        format!(" {:<10}  {:<10}  {:<10}", stars, forks, issues)
    );

    if show_files == true {
        repo_files(owner, name).await;
    }

    Ok(())
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
            Ok(repository) => {
                for item in repository {
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
        }
    }
}
