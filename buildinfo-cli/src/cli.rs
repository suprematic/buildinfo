use std::sync::LazyLock;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    pub verbose: bool,

    /// paths to project directories
    pub paths: Vec<String>,
}

pub static CLI: LazyLock<Cli> = LazyLock::new(|| Cli::parse());
