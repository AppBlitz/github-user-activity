use clap::Parser;
use commands::{
    cli::CommandUsername,
    core::api::get_data,
    infra::pdf::{list_repository, response_pdf},
};
use reqwest::blocking;

fn main() {
    // Variable which have paser commands of what user enter for CLI
    let args = CommandUsername::parse();
    // Creation client htto for request
    let client: blocking::Client = reqwest::blocking::Client::new();
    match get_data(&client, args.user_name_github) {
        Ok(value) => {
            if args.creation_pdf != false && args.name_repository.len() > 0 {
                response_pdf(&args.name_repository, &value);
            } else if args.name_repository.len() == 0 && args.creation_pdf != false {
            } else {
                println!("{:?}", list_repository(&value));
            }
        }
        Err(err) => {
            eprintln!("{err:?}")
        }
    }
}
