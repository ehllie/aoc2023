use std::{env::current_dir, path::PathBuf};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dotenv::var;

use aoc2023::inputs::AocInputs;
use aoc2023::solutions::day_solutions;

fn criterion_benchmark(c: &mut Criterion) {
    let session = var("SESSION").unwrap_or_default();
    let cache_dir = var("CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| current_dir().expect("can't get current dir").join(".cache"));

    let inputs = AocInputs::new(cache_dir, session).unwrap();
    let inputs = (1..=11).filter_map(|day| inputs.get_input(day).ok().map(|i| (i, day)));

    for (input, day) in inputs {
        let (part_one, part_two) = day_solutions(day).unwrap();
        c.bench_function(format!("day {} part 1", day).as_str(), |b| {
            b.iter(|| part_one(black_box(&input)))
        });
        c.bench_function(format!("day {} part 2", day).as_str(), |b| {
            b.iter(|| part_two(black_box(&input)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
