use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "neuralcoder3";
    let api_url = format!("https://api.github.com/users/{}/repos", username);
    let marker = "gh-pages";
    let title = "## Current Github-Pages Websites";
    let readme_file = "../../README.md";

    // retrieve all repos (iterate ?page=i until empty)
    let mut repos: Vec<serde_json::Value> = vec![];
    let mut page = 1;
    loop {
        let url = format!("{}?page={}", api_url, page);
        // needs user-agent header to avoid 403
        let response = reqwest::blocking::Client::new()
            .get(&url)
            .header("User-Agent", "reqwest")
            .send()?
            .json::<Vec<serde_json::Value>>()?;
        if response.is_empty() {
            break;
        }
        repos.extend(response);
        page += 1;
    }

    // filter out repos that have a gh-pages branch (has_pages = true)
    let gh_pages_repos: Vec<&serde_json::Value> = repos
        .iter()
        .filter(|repo| repo["has_pages"].as_bool().unwrap_or(false))
        .collect();

    // create a markdown list of links to the websites
    let mut links: Vec<String> = vec![];
    for repo in gh_pages_repos {
        // hide non-public repos
        if repo["visibility"].as_str().unwrap_or("") != "public" {
            continue;
        }
        let mut url = format!(
            "https://{}.github.io/{}",
            username,
            repo["name"].as_str().unwrap_or("")
        );
        if let Some(homepage) = repo["homepage"].as_str() {
            if !homepage.is_empty() {
                url = homepage.to_string();
            }
        }
        // remove surrounding quotes
        let repo_url = repo["html_url"].as_str().unwrap_or("").to_string();
        let repo_name = repo["name"].as_str().unwrap_or("").to_string();
        let web_url = url.trim_matches('"').to_string();
        let mut link_text = format!("- [(Repo)]({}) [{}]({})", repo_url, repo_name, web_url);
        if let Some(description) = repo["description"].as_str() {
            if !description.is_empty() {
                let description = description.replace("\n", " ");
                link_text.push_str(&format!(": {}", description));
            }
        }
        links.push(link_text);
    }

    // update the README.md
    let start_marker = format!("<!-- {} start -->", marker);
    let end_marker = format!("<!-- {} end -->", marker);
    let readme = fs::read_to_string(readme_file)?;
    let start = readme.find(&start_marker).unwrap_or_else(|| readme.len()) + start_marker.len() + 1;
    let end = readme.find(&end_marker).unwrap_or_else(|| start);
    let mut new_readme = String::new();
    if !title.is_empty() {
        new_readme.push_str(title);
        new_readme.push_str("\n\n");
    }
    new_readme.push_str(&links.join("\n"));
    let content = new_readme.trim();
    new_readme = format!("{}{}\n{}", &readme[..start], content, &readme[end..]);
    fs::write(readme_file, new_readme)?;

    Ok(())
}
