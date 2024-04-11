use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("parsing CLI");
    let args = cli::Cli::parse();
    println!("CLI parsed");

    match args.options {
        cli::Commands::Search(repository) => {
            commands::search::search_repos(
                repository.name.as_str(),
                repository.page,
                repository.per_page,
            )
            .await.expect("has occurred a error searching for repositories")
        }

        cli::Commands::Readme(repository) => {
            commands::get_readme(
                repository.owner.as_str(),
                repository.name.as_str()
            ).await.expect("has ocurred a error reading the readme, verify that the repository and readme exists")
        }
        cli::Commands::Repo(repoinfo) => {
            commands::repo::repo_information(
                repoinfo.owner.as_str(),
                repoinfo.name.as_str(),
                repoinfo.show_files,
            )
            .await
        }
    }

    Ok(())
}

pub mod cli;
pub mod commands;
