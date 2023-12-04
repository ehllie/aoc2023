use std::collections::{HashSet, VecDeque};

pub fn part_one(input: String) -> String {
    input
        .lines()
        .fold(0, |acc, l| {
            let (guesses, answers) = parse_card(l);
            let intersections = guesses.intersection(&answers).collect::<HashSet<_>>().len();
            if intersections > 0 {
                acc + 2usize.pow(intersections as u32 - 1)
            } else {
                acc
            }
        })
        .to_string()
}

pub fn part_two(input: String) -> String {
    let mut copies = VecDeque::new();
    input
        .lines()
        .fold(0, |acc, l| {
            let (guesses, answers) = parse_card(l);
            let intersections = guesses.intersection(&answers).collect::<HashSet<_>>().len();
            let current_copies = copies.pop_front().unwrap_or(1);
            if intersections > 0 {
                for i in 0..intersections {
                    if let Some(c) = copies.get_mut(i) {
                        copies[i] = *c + current_copies;
                    } else {
                        copies.push_back(current_copies + 1);
                    };
                }
            }
            current_copies + acc
        })
        .to_string()
}

fn nums_in_str(s: &str) -> HashSet<String> {
    let mut nums = HashSet::new();
    let mut num = String::new();
    for c in s.chars() {
        if c.is_numeric() {
            num.push(c);
        } else if !num.is_empty() {
            nums.insert(num.clone());
            num.clear();
        }
    }
    if !num.is_empty() {
        nums.insert(num);
    }
    nums
}

fn parse_card(line: &str) -> (HashSet<String>, HashSet<String>) {
    let mut parts = line.split(" | ");
    let guesses = nums_in_str(parts.next().unwrap().split(':').skip(1).next().unwrap());
    let answers = nums_in_str(parts.next().unwrap());
    (guesses, answers)
}
