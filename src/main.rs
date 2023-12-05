mod cli;
mod inputs;
use cli::Cli;
use inputs::AocInputs;

mod solutions;
use solutions::{
    d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12, d13, d14, d15, d16, d17, d18, d19,
    d20, d21, d22, d23, d24, d25,
};

use std::env::current_dir;
use std::fs;
use std::hint::black_box;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Result};
use clap::Parser;
use dotenv::var;

type SolutionBox = Box<dyn Fn(&str) -> String>;

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
        bench,
        passes,
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

    let (part_one, part_two): (SolutionBox, SolutionBox) = match day {
        1 => (Box::new(d01::part_one), Box::new(d01::part_two)),
        2 => (Box::new(d02::part_one), Box::new(d02::part_two)),
        3 => (Box::new(d03::part_one), Box::new(d03::part_two)),
        4 => (Box::new(d04::part_one), Box::new(d04::part_two)),
        5 => (Box::new(d05::part_one), Box::new(d05::part_two)),
        6 => (Box::new(d06::part_one), Box::new(d06::part_two)),
        7 => (Box::new(d07::part_one), Box::new(d07::part_two)),
        8 => (Box::new(d08::part_one), Box::new(d08::part_two)),
        9 => (Box::new(d09::part_one), Box::new(d09::part_two)),
        10 => (Box::new(d10::part_one), Box::new(d10::part_two)),
        11 => (Box::new(d11::part_one), Box::new(d11::part_two)),
        12 => (Box::new(d12::part_one), Box::new(d12::part_two)),
        13 => (Box::new(d13::part_one), Box::new(d13::part_two)),
        14 => (Box::new(d14::part_one), Box::new(d14::part_two)),
        15 => (Box::new(d15::part_one), Box::new(d15::part_two)),
        16 => (Box::new(d16::part_one), Box::new(d16::part_two)),
        17 => (Box::new(d17::part_one), Box::new(d17::part_two)),
        18 => (Box::new(d18::part_one), Box::new(d18::part_two)),
        19 => (Box::new(d19::part_one), Box::new(d19::part_two)),
        20 => (Box::new(d20::part_one), Box::new(d20::part_two)),
        21 => (Box::new(d21::part_one), Box::new(d21::part_two)),
        22 => (Box::new(d22::part_one), Box::new(d22::part_two)),
        23 => (Box::new(d23::part_one), Box::new(d23::part_two)),
        24 => (Box::new(d24::part_one), Box::new(d24::part_two)),
        25 => (Box::new(d25::part_one), Box::new(d24::part_two)),
        _ => bail!("Invalid day"),
    };

    if bench {
        let elapsed = if first_part {
            simple_benchmark(|| _ = black_box(part_one(&input)), passes)
        } else if second_part {
            simple_benchmark(|| _ = black_box(part_two(&input)), passes)
        } else {
            simple_benchmark(
                || {
                    _ = black_box(part_one(&input));
                    _ = black_box(part_two(&input))
                },
                passes,
            )
        };
        println!("Ran {passes} passes in {}ms", elapsed.as_millis());
        let per_pass = elapsed / passes;
        println!("{}µs/pass", per_pass.as_micros());
    } else if first_part {
        println!("{}", part_one(&input));
    } else if second_part {
        println!("{}", part_two(&input));
    } else {
        println!("{}", part_one(&input));
        println!("{}", part_two(&input));
    }
    Ok(())
}
