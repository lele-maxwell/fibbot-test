use reqwest::Error;
use serde::Deserialize;
use tokio::main;

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub title: String,
    pub body: String,
    pub html_url: String,
}

pub async fn fetch_pull_request(owner: &str, repo: &str, pr_number: u64) -> Result<PullRequest, Error> {
    let url = format!("https://api.github.com/repos/{}/{}/pulls/{}", owner, repo, pr_number);
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "rust-reqwest")
        .send()
        .await?
        .json::<PullRequest>()
        .await?;
    Ok(response)
}

pub async fn fetch_all_pull_requests(owner: &str, repo: &str) -> Result<Vec<PullRequest>, Error> {
    let url = format!("https://api.github.com/repos/{}/{}/pulls", owner, repo);
    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "rust-reqwest")
        .send()
        .await?
        .json::<Vec<PullRequest>>()
        .await?;
    Ok(response)
}

pub fn verify_pull_request(pr: &PullRequest) -> bool {
    // Check for required fields
    if pr.title.is_empty() || pr.body.is_empty() || pr.html_url.is_empty() {
        return false;
    }

    // Additional validation (e.g., specific keywords in the body)
    let required_keywords = vec!["fix", "bug", "feature"];
    for keyword in required_keywords {
        if !pr.body.to_lowercase().contains(keyword) {
            return false;
        }
    }

    true
}
