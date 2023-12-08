use std::collections::HashMap;

pub fn part_one(input: &str) -> String {
    let (steps, connections) = parse(input);
    let mut current_node = "AAA";
    let mut visits = 0;
    for dir in steps.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }

        let (l, r) = connections.get(current_node).unwrap();
        visits += 1;
        if dir == &Direction::L {
            current_node = l;
        } else {
            current_node = r;
        }
    }

    visits.to_string()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part_two(input: &str) -> String {
    let (steps, connections) = parse(input);
    let start_nodes = connections.keys().filter(|node| node.ends_with('A'));
    let cycle_lengts = start_nodes.map(|node| {
        let mut current_node = *node;
        let mut visits = 0;

        for dir in steps.iter().cycle() {
            if current_node.ends_with('Z') {
                break;
            }

            let (l, r) = connections.get(current_node).unwrap();
            visits += 1;
            if dir == &Direction::L {
                current_node = l;
            } else {
                current_node = r;
            }
        }
        visits
    });

    cycle_lengts.reduce(lcm).unwrap().to_string()
}

#[derive(Debug, PartialEq)]
enum Direction {
    L,
    R,
}

fn parse(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    match input.split("\n\n").collect::<Vec<_>>()[..] {
        [steps, connections] => (
            steps
                .chars()
                .filter_map(|c| match c {
                    'L' => Some(Direction::L),
                    'R' => Some(Direction::R),
                    _ => None,
                })
                .collect(),
            connections
                .lines()
                .filter_map(|l| match l.split(" = ").collect::<Vec<_>>()[..] {
                    [node, connections] => {
                        let mut connections = connections[1..connections.len() - 1].split(", ");
                        Some((node, (connections.next()?, connections.next()?)))
                    }
                    _ => None,
                })
                .collect(),
        ),
        _ => panic!("invalid input"),
    }
}
