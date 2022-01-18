use clap::Parser;

#[derive(Parser, Debug)]
pub struct CLI {
    #[clap(default_value = "init")]
    pub command: String,
    #[clap(default_value = "")]
    pub project_name: String,
}
