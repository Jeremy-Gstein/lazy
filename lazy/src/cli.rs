use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "lazy", version, about, color = clap::ColorChoice::Always)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg()]
    pub sequence: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Rm,
    Run,
    Build,
}

