use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    pub id: String,
    #[serde(rename = "type")]
    pub type_event: String,
    pub actor: Actor,
    pub repo: Repo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    pub id: i64,
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub url: String,
}
