#[tokio::main]
async fn main() -> Result<(), ()> {
    let owner = "NatProgramer";
    let repository = "repoinfo";

    commands::get_readme(owner, repository).await;
    // commands::search::search_repos(repository, 1, 15).await;

    Ok(())
}

pub mod commands;
