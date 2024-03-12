use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "repoinfo")]
#[command(
    version = "1.0.0",
    // about = "A CLI for make somethings like search, view and more, lightheight and with some insteresting features"
)]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    option: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // #[command(about = "Search for different things like repositories, issues and Pull requests")]
    Search(Repository),
}

#[derive(Debug, Args)]
struct Repository {
    #[arg(value_name = "NAME")]
    name: String,
    #[arg(required = false, default_missing_value = "0", value_name = "PAGE")]
    page: u64,
    #[arg(
        required = false,
        default_missing_value = "15",
        value_name = "NUM_PER_PAGE"
    )]
    per_page: u64,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Cli::parse();

    match args.option {
        Commands::Search(repository) => {
            commands::search::search_repos(repository.name.as_str(), 0, 15).await;
        }
    }

    // commands::get_readme(owner, repository).await;
    // commands::repo::repo_information(owner, repository).await;

    Ok(())
}

pub mod commands;
