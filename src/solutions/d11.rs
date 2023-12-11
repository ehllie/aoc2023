use std::collections::HashMap;

type Galaxies = Vec<(isize, isize)>;

pub fn part_one(input: &str) -> String {
    let galaxies = parse_universe(input, 2);
    galaxy_distances(&galaxies).to_string()
}

pub fn part_two(input: &str) -> String {
    let galaxies = parse_universe(input, 1_000_000);
    galaxy_distances(&galaxies).to_string()
}

fn galaxy_distances(galaxies: &Galaxies) -> isize {
    galaxies.iter().enumerate().fold(0, |acc, (i, (x1, y1))| {
        galaxies.iter().skip(i + 1).fold(acc, |acc, (x2, y2)| {
            let manhatan = (x1 - x2).abs() + (y1 - y2).abs();
            acc + manhatan
        })
    })
}

fn parse_universe(input: &str, expansion_rate: isize) -> Galaxies {
    let mut columns = HashMap::<isize, Vec<(isize, isize, isize)>>::new();
    let mut expanded_rows = 0;
    let mut row_length = 0;
    for (row, line) in input.lines().enumerate() {
        let mut empty_row = true;
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                if let Some(column) = columns.get_mut(&(col as isize)) {
                    column.push((row as isize, 0, expanded_rows));
                } else {
                    columns.insert(col as isize, Vec::from([(row as isize, 0, expanded_rows)]));
                }
                empty_row = false;
            }
        }
        row_length = line.len() as isize;
        if empty_row {
            expanded_rows += 1;
        }
    }

    let mut expanded_columns = 0;
    for col in 0..row_length {
        if let Some(column) = columns.get_mut(&col) {
            for row in column.iter_mut() {
                *row = (row.0, expanded_columns, row.2);
            }
        } else {
            expanded_columns += 1;
        }
    }

    let expansion_rate = expansion_rate - 1;
    columns
        .into_iter()
        .flat_map(|(col, rows)| {
            rows.into_iter().map(move |(row, e_col, e_row)| {
                (col + e_col * expansion_rate, row + e_row * expansion_rate)
            })
        })
        .collect()
}
