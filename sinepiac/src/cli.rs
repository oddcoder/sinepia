use clap::Parser;

/// Sinepia compiler
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// file to compile
    pub input: String,
    /// Sets the level of verbosity (use -v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbosity: u8,
}
