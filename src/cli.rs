use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "repoinfo")]
#[command(
    version = "0.1.0",
    about = "A CLI for make somethings like search, view and more, lightheight and with some insteresting features"
)]
#[command(long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub options: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Search(RepositoryArgs),
    Readme(Repository),
    Repo(Repository),
}

#[derive(Debug, Args)]
pub struct RepositoryArgs {
    #[arg(value_name = "NAME")]
    pub name: String,

    #[arg(required = false, default_missing_value = "0", value_name = "PAGE")]
    pub page: u32,

    #[arg(
        required = false,
        default_missing_value = "15",
        value_name = "NUM_PER_PAGE"
    )]
    pub per_page: u32,
}

#[derive(Debug, Args)]
pub struct Repository {
    #[arg(value_name = "Owner")]
    // The owner of the repository
    pub owner: String,

    #[arg(value_name = "Name")]
    // The repository name
    pub name: String,

    #[arg(long = "show-files")]
    // This option enable show files of the repository
    pub show_files: bool,
}