use clap::Parser;
use commands::{cli::CommandUsername, core::api::get_data, infra::functions::response_pdf};
use reqwest::blocking;

fn main() {
    let args = CommandUsername::parse();
    let client: blocking::Client = reqwest::blocking::Client::new();
    if let Ok(value) = get_data(&client, args.user_name_github) {
        response_pdf(&args.name_repository, &value);
    }
}
