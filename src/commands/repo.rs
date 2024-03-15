use reqwest::Client;
use std::process;

pub async fn repo_information(owner: &str, name: &str) {
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
                println!("Homepage: {}", homepage)
            } else {
            }

            let issues = json_response.open_issues;
            let forks = json_response.forks;
            let stars = json_response.stargazers_count;

            println!(
                "{}",
                format!(" {:<10}  {:<10}  {:<10}", stars, forks, issues)
            )
        }
        Err(err) => {
            eprintln!("Has ocurred a error while getting repository information");
            eprintln!("Error: {}", err);
            process::exit(1)
        }
    }
}
