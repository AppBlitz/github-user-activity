use clap::Parser;
use commands::{cli::CommandUsername, core::api::get_user, infra::functions::response_pdf};

fn main() {
    let args = CommandUsername::parse();
    if let Ok(value) = get_user(args.user_name_github) {
        response_pdf(&args.name_repository, &value);
    }
}
