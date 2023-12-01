use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Day for which solution is to be run
    pub day: u8,

    /// Whether to run only the first part of the solution
    #[arg(short, long = "first", group = "part")]
    pub first_part: bool,

    /// Whether to run only the second part of the solution
    #[arg(short, long = "second", group = "part")]
    pub second_part: bool,

    /// Example input file
    #[arg(short, long)]
    pub example: Option<PathBuf>,
}
