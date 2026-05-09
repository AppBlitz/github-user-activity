use reqwest::blocking::Client;

use crate::{core::api::get_data, infra::pdf::generate_pdf, model::github_events::Events};

pub fn response_vec_string(client: &Client, name_user: String) {
    let vec_receive = get_data(client, name_user).unwrap();
    let response = search_name_respository(&vec_receive);
    match generate_pdf(&response, &vec_receive) {
        Ok(message) => {
            println!("{message:?}")
        }
        Err(err) => panic!("{:?}", err),
    }
}

fn search_name_respository(vector_repository: &Vec<Events>) -> Vec<String> {
    let mut vec_response: Vec<String> = Vec::new();
    for value in vector_repository {
        if !vec_response.contains(&value.repo.name) {
            vec_response.push(value.repo.name.to_owned());
        }
    }
    vec_response
}
