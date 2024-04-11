use reqwest::Client;

pub async fn search_repos(repository: &str, page: u32, per_page: u32) -> Result<(), reqwest::Error> {
    let base_url = "https://api.github.com";
    let client = Client::new();

    let response = client
        .get(format!(
            "{base_url}/search/repositories?q={repository}&per_page={per_page}&page={page}"
        ))
        .header("User-Agent", repository)
        .send()
        .await?;

    let repo_list = response.json::<super::RepoSearchResult>().await;

    match repo_list {
        Ok(content) => {
            println!("Results: {:#?}", content.total_count);
            println!("Showed: {}", per_page);

            for repo in content.items {
                println!("{}", format!("{:->135}", ""));
                println!("Owner: {}", repo.owner.login);
                println!(
                    "Name: {name} {main_branch} {desc} {stars} {issues}  {forks}",
                    name = if repo.name.len() > 10 {
                        format!("{:<20}", format!("{}...", &repo.name[0..10]))
                    } else {
                        format!("{:<20}", repo.name)
                    },
                    main_branch = format!("{:<30}", format!("󰘬 {}", repo.default_branch)),
                    desc = match repo.description {
                        Some(description) => {
                            if description.len() > 25 {
                                format!(
                                    "{:<50}",
                                    format!("{}...", &description[0..25].trim())
                                )
                            } else {
                                format!("{:<50}", description)
                            }
                        }
                        None => {
                            String::from(format!("{:<50}", "No description"))
                        }
                    },
                    stars = format!("{:<10}", format!(" {}", repo.stargazers_count)),
                    forks = format!("{:<10}", format!(" {}", repo.forks)),
                    issues = format!("{:<10}", format!(" {}", repo.open_issues))
                );

                match repo.homepage {
                    Some(homepage) => {
                        if homepage.len() > 0 {
                            println!("Homepage 󰋜 : {}", homepage)
                        }
                    }
                    None => (),
                }
            }
        }
        Err(err) => {
            eprintln!("The program has exited with error: {}", err);
        }
    };

    Ok(())
}
