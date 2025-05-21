use std::path::PathBuf;

#[derive(clap::Parser)]
pub struct Cli {
    /// This is a required slot for the Python executable path
    py: String,

    /// Path to JSON with custom responses
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
}
