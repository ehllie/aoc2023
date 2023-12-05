struct Handful {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

type Game = (u32, Vec<Handful>);

pub fn part_one(input: &str) -> String {
    input
        .lines()
        .map(parse_game)
        .filter_map(|(id, hs)| {
            if hs
                .iter()
                .all(|h| 12 >= h.red && 13 >= h.green && 14 >= h.blue)
            {
                Some(id)
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    input
        .lines()
        .map(parse_game)
        .map(|(_, hs)| {
            let min = hs.iter().fold(
                Handful {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |acc, el| Handful {
                    red: acc.red.max(el.red),
                    green: acc.green.max(el.green),
                    blue: acc.blue.max(el.blue),
                },
            );
            min.red * min.green * min.blue
        })
        .sum::<u32>()
        .to_string()
}

fn parse_game(line: &str) -> Game {
    let mut parts = line.split(": ");
    let header = parts.next().unwrap();
    let handfuls = parts.next().unwrap();
    let id = header.split(" ").last().unwrap().parse().unwrap();
    let handfuls = handfuls
        .split("; ")
        .map(|h| {
            let mut blue = 0;
            let mut red = 0;
            let mut green = 0;
            for c in h.split(", ") {
                let mut parts = c.split(" ");
                let count = parts.next().unwrap().parse().unwrap();
                match parts.next().unwrap() {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => panic!("Unknown color"),
                };
            }
            Handful { red, green, blue }
        })
        .collect();
    (id, handfuls)
}
