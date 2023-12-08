pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;
pub mod d23;
pub mod d24;
pub mod d25;

use anyhow::{bail, Result};

pub type SolutionBox = Box<dyn Fn(&str) -> String>;

pub fn day_solutions(day: u8) -> Result<(SolutionBox, SolutionBox)> {
    Ok(match day {
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
    })
}
