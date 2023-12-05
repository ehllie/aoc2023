use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Day for which a solution is to be ran
    pub day: u8,

    /// Whether to run only the first part of the solution
    #[arg(short, long = "first", group = "part")]
    pub first_part: bool,

    /// Whether to run only the second part of the solution
    #[arg(short, long = "second", group = "part")]
    pub second_part: bool,

    /// Path to input file
    #[arg(short, long = "input")]
    pub input_file: Option<PathBuf>,

    /// Benchmark the solution
    #[arg(short, long = "bench")]
    pub bench: bool,

    /// Benchmarking passes to run
    #[arg(short, long = "passes", default_value = "100")]
    pub passes: u32,
}
