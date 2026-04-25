use crate::model::github_events::Events;
use reqwest::{Result, blocking};

static CLIENTS: std::sync::LazyLock<blocking::Client> =
    std::sync::LazyLock::new(|| match blocking::Client::builder().build() {
        Ok(value) => {
            return value;
        }
        Err(err) => panic!("{err:?}"),
    });

pub fn get_user(name_user_github: String) -> Result<Vec<Events>> {
    let uri: String = format!("https://api.github.com/users/{name_user_github}/events");
    let body = CLIENTS.get(uri).send()?;
    let res: Vec<Events> = body.json()?;
    Ok(res)
}
