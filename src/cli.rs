use clap::Parser;

#[derive(Parser, Debug)]
pub struct CLI {
    #[clap(short, long)]
    pub project_name: String,
    #[clap(short, long, default_value = "build")]
    pub subcommand: String,
}
