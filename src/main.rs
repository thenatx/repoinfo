#[tokio::main]
async fn main() {
    // let owner = "thenatprogramer";
    let repository = "repoinfo";

    // commands::get_readme(owner, repository).await;
    commands::search::search_repos(repository, 1, 15).await;

    Ok::<(), ()>(()).expect("A unexpected error encountred")
}

pub mod commands;
