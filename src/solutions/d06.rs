pub fn part_one(input: &str) -> String {
    parse_times(input)
        .map(binomial_solutions)
        .product::<isize>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    binomial_solutions(parse_time(input)).to_string()
}

/// p * (t - p) > d
/// pt - p^2 > d
/// p^2 - tp + d < 0
/// p > (t - sqrt(t^2 - 4d)) / 2
/// p < (t + sqrt(t^2 - 4d)) / 2
fn binomial_solutions((time, distance): (isize, isize)) -> isize {
    let delta = time * time - 4 * distance;
    if delta > 0 {
        let delta = (delta as f64).sqrt();
        let p1 = (time as f64 - delta) / 2.;
        let p1 = p1.floor() as isize;
        let p2 = (time as f64 + delta) / 2.;
        let p2 = p2.ceil() as isize - 1;
        p2 - p1
    } else {
        0
    }
}

fn parse_times(input: &str) -> impl Iterator<Item = (isize, isize)> + '_ {
    match input.lines().collect::<Vec<_>>()[..] {
        [times, distances] => times
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .zip(distances.split(' ').filter_map(|s| s.parse().ok())),
        _ => unreachable!(),
    }
}

fn parse_time(input: &str) -> (isize, isize) {
    match input.lines().collect::<Vec<_>>()[..] {
        [time, distance] => {
            let time = time
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            let distance = distance
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            (time, distance)
        }
        _ => unreachable!(),
    }
}
