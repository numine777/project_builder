use clap::Parser;

#[derive(Parser, Debug)]
pub struct CLI {
    // #[clap(short, long, default_value = "search")]
    pub project_name: String,
    #[clap(short, long, default_value = "run")]
    pub subcommand: String,
}
