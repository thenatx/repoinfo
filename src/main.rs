use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "repoinfo")]
#[command(
    version = "1.0.0",
    about = "A CLI for make somethings like search, view and more, lightheight and with some insteresting features"
)]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    option: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Search(RepositoryArgs),
    Readme(Repository),
    Repo(Repository),
}

#[derive(Debug, Args)]
struct RepositoryArgs {
    #[arg(value_name = "NAME")]
    name: String,
    #[arg(required = false, default_missing_value = "0", value_name = "PAGE")]
    page: u32,
    #[arg(
        required = false,
        default_missing_value = "15",
        value_name = "NUM_PER_PAGE"
    )]
    per_page: u32,
}
#[derive(Debug, Args)]
struct Repository {
    #[arg(value_name = "Owner")]
    owner: String,

    #[arg(value_name = "Name")]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Cli::parse();

    match args.option {
        Commands::Search(repository) => {
            commands::search::search_repos(
                repository.name.as_str(),
                repository.page,
                repository.per_page,
            )
            .await
        }

        Commands::Readme(repository) => {
            commands::get_readme(repository.owner.as_str(), repository.name.as_str()).await
        }
        Commands::Repo(repoinfo) => {
            commands::repo::repo_information(repoinfo.owner.as_str(), repoinfo.name.as_str()).await
        }
    }

    Ok(())
}

pub mod commands;
