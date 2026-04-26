use reqwest::{
    Result,
    blocking::{self, Response},
    header::USER_AGENT,
};

use crate::model::github_events::Events;

fn response_client(client: &blocking::Client, name_user_github: String) -> Result<Response> {
    let uri: String = format!("https://api.github.com/users/{name_user_github}/events");
    let response = client
        .get(uri)
        .header(USER_AGENT, "github-user-activity")
        .send()?;
    Ok(response)
}

pub fn get_data(client: &blocking::Client, name_user_github: String) -> Result<Vec<Events>> {
    let response = response_client(client, name_user_github)?;
    let body: Vec<Events> = response.json()?;
    Ok(body)
}
