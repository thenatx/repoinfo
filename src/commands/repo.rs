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

            println!("{:#?}", json_response)
        }
        Err(err) => {
            eprintln!("Has ocurred a error while getting repository information");
            eprintln!("Error: {}", err);
            process::exit(1)
        }
    }
}
