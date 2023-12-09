pub fn part_one(input: &str) -> String {
    parse(input)
        .map(|hist| {
            derivatives(hist)
                .into_iter()
                .rev()
                .fold(0, |acc, drv| acc + drv.last().unwrap())
        })
        .sum::<isize>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    parse(input)
        .map(|hist| {
            derivatives(hist)
                .into_iter()
                .rev()
                .fold(0, |acc, drv| drv.first().unwrap() - acc)
        })
        .sum::<isize>()
        .to_string()
}

fn derivatives<I: Iterator<Item = isize>>(iter: I) -> Vec<Vec<isize>> {
    let mut derivatives = vec![iter.collect::<Vec<_>>()];
    while !derivatives.last().unwrap().iter().all(|n| n == &0) {
        let derivative = derivatives
            .last()
            .unwrap()
            .windows(2)
            .map(|win| win[1] - win[0]);
        derivatives.push(derivative.collect());
    }
    derivatives
}

fn parse(input: &str) -> impl Iterator<Item = impl Iterator<Item = isize> + '_> + '_ {
    input
        .lines()
        .map(|l| l.split(' ').filter_map(|s| s.parse().ok()))
}
