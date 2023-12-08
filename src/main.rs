mod cli;
mod inputs;
use cli::Cli;
use inputs::AocInputs;

mod solutions;
use solutions::day_solutions;

use std::env::current_dir;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use clap::Parser;
use dotenv::var;

fn simple_benchmark<F: Fn()>(func: F, passes: u32) -> Duration {
    let start = Instant::now();
    for _ in 0..passes {
        func();
    }
    start.elapsed()
}

pub fn main() -> Result<()> {
    let Cli {
        day,
        first_part,
        second_part,
        input_file,
    } = Cli::parse();

    let input = match input_file {
        Some(path) => fs::read_to_string(path)?,
        None => {
            let session =
                var("SESSION").map_err(|_| anyhow!("SESSION environment variable not found"))?;
            let cache_dir = match var("CACHE_DIR") {
                Ok(dir) => PathBuf::from(dir),
                Err(_) => current_dir()?.join(".cache"),
            };

            let inputs = AocInputs::new(cache_dir, session)?;
            inputs.get_input(day)?
        }
    };

    let (part_one, part_two) = day_solutions(day)?;

    if first_part {
        println!("{}", part_one(&input));
    } else if second_part {
        println!("{}", part_two(&input));
    } else {
        println!("{}", part_one(&input));
        println!("{}", part_two(&input));
    }
    Ok(())
}
