#[tokio::main]
async fn main() {
    let owner = "NatProgramer";
    let repository = "task-manager-server";


    commands::get_readme(owner, repository).await;
    commands::search_repos(repository, 1).await;

    Ok::<(), ()>(()).expect("A unexpected error encountred")
}

pub mod commands;