use crate::model::github_events::Events;
use reqwest::{Result, blocking};
pub fn get_user(name_user_github: String) -> Result<Vec<Events>> {
    let uri: String = format!("https://api.github.com/users/{name_user_github}/events");
    let client = blocking::Client::builder()
        .user_agent("github-user-activity")
        .build()?;
    let body = client.get(uri).send()?;
    let res: Vec<Events> = body.json()?;
    Ok(res)
}
