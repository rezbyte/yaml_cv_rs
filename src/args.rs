//! Contains the code for handling CLI arguments.
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// The standard arguments for the CLI.
pub(crate) struct Args {
    /// Path to the input file in YAML format.
    #[arg(short, long, default_value = "data.yaml")]
    pub(crate) input: PathBuf,

    /// Path to the styling file.
    #[arg(short, long, default_value = "style.txt")]
    pub(crate) style: PathBuf,

    /// Path to output the final PDF file to.
    #[arg(short, long, default_value = "output.pdf")]
    pub(crate) output: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
