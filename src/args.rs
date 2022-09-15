//! Contains the code for handling CLI arguments.
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// The standard arguments for the CLI.
pub(crate) struct Args {
    /// Path to the input file in YAML format.
    #[clap(short, long, value_parser, default_value = "data.yaml")]
    input: PathBuf,

    /// Path to the styling file.
    #[clap(short, long, value_parser, default_value = "style.txt")]
    style: PathBuf,

    /// Path to output the final PDF file to.
    #[clap(short, long, value_parser, default_value = "output.pdf")]
    output: PathBuf,
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
