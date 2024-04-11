use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = cli::Cli::parse();

    match args.options {
        cli::Commands::Search(repository) => {
            commands::search::search_repos(
                &repository.name,
                repository.page,
                repository.per_page,
            )
            .await.expect("has occurred a error searching for repositories")
        }

        cli::Commands::Readme(repository) => {
            commands::get_readme(
                &repository.owner,
                &repository.name
            ).await.expect("Has ocurred a error reading the readme, verify that the repository exists and have a README.md")
        }
        cli::Commands::Repo(repoinfo) => {
            commands::repo::repo_information(
                &repoinfo.owner,
                &repoinfo.name,
                repoinfo.show_files,
            )
            .await.expect("Has occurred a error while getting repository information, check that the repository exists and is public")
        }
    }

    Ok(())
}

pub mod cli;
pub mod commands;
