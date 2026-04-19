use clap::Parser;

#[derive(Parser, Debug)]
#[doc = "This struct for creating commands"]
pub struct CommandUsername {
    #[arg(short, long)]
    pub user_name_github: String,

    #[arg(short, long, required = false)]
    pub creation_pdf: bool,

    #[arg(short, long,num_args=1..)]
    pub name_repository: Vec<String>,
}
