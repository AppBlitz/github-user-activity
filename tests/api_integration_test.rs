use commands::{core::api::get_data, infra::pdf::create_document};

#[test]
#[doc = "Test for verification the creation the document to save information of user"]
fn verification_to_create_document() {
    let result = create_document();
    assert!(result.is_ok())
}

#[test]
#[doc = "test verification recive information of request to API github"]
fn verification_body_request_api_github() {
    let client = reqwest::blocking::Client::new();
    let result = get_data(&client, String::from("AppBlitz")).unwrap();
    assert!(result.len() > 0)
}
