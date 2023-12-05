pub fn part_one(input: &str) -> String {
    input
        .lines()
        .map(|l| digits_in_line(l, &NUM_DIGITS))
        .filter_map(connect_ends)
        .sum::<u32>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    input
        .lines()
        .map(|l| digits_in_line(l, &Vec::from([NUM_DIGITS, WORD_DIGITS]).concat()))
        .filter_map(connect_ends)
        .sum::<u32>()
        .to_string()
}

fn digits_in_line(line: &str, dict: &[(&str, u32)]) -> Vec<u32> {
    tails(line).filter_map(|t| start_digit(t, dict)).collect()
}

fn tails(s: &str) -> impl Iterator<Item = &str> {
    (0..s.len()).map(|i| &s[i..s.len()])
}

fn start_digit(line: &str, dict: &[(&str, u32)]) -> Option<u32> {
    dict.iter()
        .find_map(|(prefix, val)| {
            if line.starts_with(prefix) {
                Some(val)
            } else {
                None
            }
        })
        .copied()
}

fn connect_ends(digits: Vec<u32>) -> Option<u32> {
    Some(digits.first()? * 10 + digits.last()?)
}

const WORD_DIGITS: [(&str, u32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

const NUM_DIGITS: [(&str, u32); 10] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];
