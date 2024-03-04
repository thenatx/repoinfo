use serde::Deserialize;

#[derive(Deserialize)]
struct ReadmeRes {
    name: String,
}

#[tokio::main]
async fn main() {
    let user = "NatProgramer";
    let repository = "task-manager-server";

    Ok(())
}

async fn get_readme(owner: &str, repository: &str) {
    let client = reqwest::Client::new();

    let res = client
        .get(
            format!("https://api.github.com/repos/{owner}/{repository}/readme")
        )
        .bearer_auth(GITHUB_TOKEN)
        .header("User-Agent", user)
        .send().await;
    
    match res {
        Ok(response) => println!("{:#?}", response.json::<ReadmeRes>().await.unwrap().name),
        Err(error) => {
            println!("{}", error);
            std::process::exit(1)
        }
    }
}
