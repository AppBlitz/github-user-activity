use clap::Parser;

#[derive(Parser, Debug)]
#[doc = "This struct for creating commands"]
#[command(version = "1.0.1")]
pub struct CommandUsername {
    // name repository user
    #[arg(short, long)]
    pub user_name_github: String,

    // Option to create pdf
    #[arg(short, long, required = false)]
    pub creation_pdf: bool,

    // name fo repository for verification
    #[arg(short, long,num_args=1..)]
    pub name_repository: Vec<String>,
}
