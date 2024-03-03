#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let readme = octocrab::instance()
        .repos("NatProgramer", "repoinfo")
        .get_readme()
        .r#ref("main").send().await;

    println!("{:#?}", readme.unwrap());
}
