mod cli;
mod inputs;
use cli::Cli;
use inputs::AocInputs;

mod solutions;
use solutions::{d1, d2};

use std::env::current_dir;
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Parser;
use dotenv::var;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let Cli {
        day,
        first_part,
        second_part,
        example,
    } = Cli::parse();

    let input = match example {
        Some(path) => {
            let mut file = File::open(path).await?;
            let mut input = String::new();
            file.read_to_string(&mut input).await?;
            input
        }
        None => {
            let session = var("SESSION")?;
            let cache_dir = match var("CACHE_DIR") {
                Ok(dir) => PathBuf::from(dir),
                Err(_) => current_dir()?.join(".cache"),
            };

            let inputs = AocInputs::new(cache_dir, session)?;
            inputs.get_input(day).await?
        }
    };

    let (part_one, part_two): (
        Box<dyn FnOnce(String) -> String>,
        Box<dyn FnOnce(String) -> String>,
    ) = match day {
        1 => (Box::new(d1::part_one), Box::new(d1::part_two)),
        2 => (Box::new(d2::part_one), Box::new(d2::part_two)),
        _ => bail!("Invalid day"),
    };

    if first_part {
        println!("{}", part_one(input));
    } else if second_part {
        println!("{}", part_two(input));
    } else {
        println!("{}", part_one(input.clone()));
        println!("{}", part_two(input));
    }
    Ok(())
}
