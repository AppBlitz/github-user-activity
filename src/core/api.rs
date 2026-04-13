use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};

#[tokio::main]
#[allow(unused_variables)]
pub async fn get_user(name_user_github: String) {
    let mut uri: String = String::from("https://api.github.com/users/");
    uri.push_str(name_user_github.as_str());
    uri.push_str("/events");

    let client = reqwest::Client::new();
    let body = client
        .get(uri)
        .header(USER_AGENT, "github-user-activity")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{:?}", body)
}
