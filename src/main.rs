use clap::Parser;
use commands::{
    cli::CommandUsername,
    core::api::get_data,
    infra::functions::{list_repository, response_pdf},
};
use reqwest::blocking;

fn main() {
    let args = CommandUsername::parse();
    let client: blocking::Client = reqwest::blocking::Client::new();
    match get_data(&client, args.user_name_github) {
        Ok(value) => {
            if args.creation_pdf != false {
                println!("{:?}", &args.name_repository);
                response_pdf(&args.name_repository, &value);
            } else {
                println!("{:?}", list_repository(&value))
            }
        }
        Err(err) => {
            eprintln!("{err:?}")
        }
    }
}
