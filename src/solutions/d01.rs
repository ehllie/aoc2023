pub fn part_one(input: String) -> String {
    let sum_vals: u32 = input
        .lines()
        .map(digits_in_line)
        .filter_map(connect_ends)
        .sum();
    format!("{}", sum_vals)
}

pub fn part_two(input: String) -> String {
    let sum_vals: u32 = input
        .lines()
        .map(spelled_digits)
        .filter_map(connect_ends)
        .sum();
    format!("{}", sum_vals)
}

fn connect_ends(digits: Vec<u32>) -> Option<u32> {
    let mut iter = digits.iter();
    let first = iter.next();
    let last = iter.last().or(first);
    Some(first? * 10 + last?)
}

fn digits_in_line(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn start_digit(line: &str) -> Option<u32> {
    if line.starts_with("one") {
        return Some(1);
    };
    if line.starts_with("two") {
        return Some(2);
    };
    if line.starts_with("three") {
        return Some(3);
    };
    if line.starts_with("four") {
        return Some(4);
    };
    if line.starts_with("five") {
        return Some(5);
    };
    if line.starts_with("six") {
        return Some(6);
    };
    if line.starts_with("seven") {
        return Some(7);
    };
    if line.starts_with("eight") {
        return Some(8);
    };
    if line.starts_with("nine") {
        return Some(9);
    };
    line.chars().next().and_then(|c| c.to_digit(10))
}

fn spelled_digits(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    for i in 0..line.len() {
        if let Some(digit) = start_digit(&line[i..line.len()]) {
            digits.push(digit);
        }
    }
    digits
}
