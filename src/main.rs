use clap::Command;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let command = Command::new("repoinfo")
        .about("Get search repositories, get information of repositories and more!");

    let args = command.args([]);

    println!("{}", args);

    // let owner = "pheralb";
    // let repository = "svgl";

    // commands::get_readme(owner, repository).await;
    // commands::search::search_repos(repository, 1, 15).await;
    // commands::repo::repo_information(owner, repository).await;

    Ok(())
}

pub mod commands;
