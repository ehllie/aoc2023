use std::collections::HashMap;

enum SchematicItem {
    Number(String),
    Symbol(char),
}

type Point = (usize, usize);

type Schematic = HashMap<Point, SchematicItem>;

pub fn part_one(input: &str) -> String {
    let schematic = parse_schematic(&input);
    let part_nums = schematic.iter().filter_map(|(p, i)| match i {
        SchematicItem::Number(n) => {
            let borders_symbol = adjacent_items(p, i).any(|p| match schematic.get(&p) {
                Some(SchematicItem::Symbol(_)) => true,
                _ => false,
            });
            if borders_symbol {
                n.parse::<u32>().ok()
            } else {
                None
            }
        }
        _ => None,
    });
    part_nums.sum::<u32>().to_string()
}

pub fn part_two(input: &str) -> String {
    let schematic = parse_schematic(&input);
    let gears = schematic.iter().filter_map(|(sp, si)| match si {
        SchematicItem::Symbol('*') => {
            let adjacent_parts = schematic
                .iter()
                .filter_map(|(np, ni)| match ni {
                    SchematicItem::Number(n) => {
                        if adjacent_items(np, ni).any(|p| p == *sp) {
                            n.parse::<u32>().ok()
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            if adjacent_parts.len() == 2 {
                Some(adjacent_parts[0] * adjacent_parts[1])
            } else {
                None
            }
        }
        _ => None,
    });
    gears.sum::<u32>().to_string()
}

fn adjacent_items<'a>(
    location: &'a Point,
    item: &'a SchematicItem,
) -> impl Iterator<Item = Point> + 'a {
    let x_min = 0.max(location.0 as isize - 1) as usize;
    let x_max = match item {
        SchematicItem::Number(n) => location.0 + n.len(),
        _ => location.0 + 1,
    };
    let y_min = 0.max(location.1 as isize - 1) as usize;
    let y_max = location.1 + 1;
    (x_min..=x_max)
        .flat_map(move |x| (y_min..=y_max).map(move |y| (x, y)))
        .filter(move |(x, y)| {
            !(y == &location.1
                && match item {
                    SchematicItem::Number(n) => x < &(location.0 + n.len()) && x >= &location.0,
                    _ => x == &location.0,
                })
        })
}

fn parse_schematic(input: &str) -> Schematic {
    let mut schematic = Schematic::new();
    let mut x = 0;
    let mut y = 0;
    let mut number = String::new();
    for c in input.chars() {
        if (c < '0' || c > '9') && !number.is_empty() {
            schematic.insert((x - number.len(), y), SchematicItem::Number(number.clone()));
            number.clear();
        }
        match c {
            '\n' => {
                x = 0;
                y += 1;
            }
            '0'..='9' => {
                number.push(c);
                x += 1;
            }
            _ if c != '.' => {
                schematic.insert((x, y), SchematicItem::Symbol(c));
                x += 1;
            }
            _ => x += 1,
        }
    }
    schematic
}
