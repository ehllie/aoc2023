use partitions::{partition_vec, PartitionVec};
use std::{collections::HashMap, rc::Rc};

pub fn part_one(input: &str) -> String {
    let graph = pipe_graph(input);
    let index_map = graph
        .iter()
        .enumerate()
        .map(|(i, (pos, _))| (pos, i))
        .collect::<HashMap<_, _>>();
    let mut pv: PartitionVec<()> = partition_vec![(); graph.len()];
    for (node, connections) in graph.iter() {
        for other_node in connections.iter() {
            match graph.get(other_node) {
                Some(connections) if connections.contains(node) => {
                    let pos = index_map[node];
                    let other_pos = index_map[other_node];
                    pv.union(pos, other_pos);
                }
                _ => {}
            }
        }
    }
    let start_node = graph
        .iter()
        .find_map(|(pos, connections)| {
            if connections.len() == 4 {
                Some(pos)
            } else {
                None
            }
        })
        .unwrap();
    let start_pos = index_map[start_node];
    (pv.len_of_set(start_pos) / 2).to_string()
}

pub fn part_two(input: &str) -> String {
    let graph = sparse_graph(input);
    let mut pv = graph.keys().collect::<PartitionVec<(isize, isize)>>();
    let index_map = pv.clone();
    let index_map = index_map
        .iter()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

    let mut queue = vec![index_map[&(0, 0)]];
    while let Some(index) = queue.pop() {
        let pos = pv[index];
        let neighbours = vec![
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
            (pos.0 + 1, pos.1 + 1),
            (pos.0 - 1, pos.1 - 1),
            (pos.0 + 1, pos.1 - 1),
            (pos.0 - 1, pos.1 + 1),
        ];

        for neighbour in neighbours {
            if let Some(&neighbour_index) = index_map.get(&neighbour) {
                let is_wall = graph[&neighbour];
                if !pv.same_set(index, neighbour_index) {
                    pv.union(index, neighbour_index);
                    if !is_wall {
                        queue.push(neighbour_index);
                    }
                }
            }
        }
    }

    let outside = index_map[&(0, 0)];

    let inside = graph.keys().filter_map(|pos| {
        if !pv.same_set(outside, index_map[pos]) && pos.0 % 3 == 1 && pos.1 % 3 == 1 {
            Some((pos.0 / 3 - 1, pos.1 / 3 - 1))
        } else {
            None
        }
    });

    inside.count().to_string()
}

fn pipe_connections(shape: char, pos: (isize, isize)) -> Box<[(isize, isize)]> {
    Box::from(match shape {
        '|' => vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
        '-' => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
        'L' => vec![(pos.0, pos.1 - 1), (pos.0 + 1, pos.1)],
        'J' => vec![(pos.0, pos.1 - 1), (pos.0 - 1, pos.1)],
        '7' => vec![(pos.0, pos.1 + 1), (pos.0 - 1, pos.1)],
        'F' => vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)],
        'S' => vec![
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ],
        _ => vec![],
    })
}

fn pipe_graph(input: &str) -> HashMap<(isize, isize), Box<[(isize, isize)]>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c != '.' {
                    Some((
                        (col as isize, row as isize),
                        pipe_connections(c, (col as isize, row as isize)),
                    ))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn sparse_graph(input: &str) -> HashMap<(isize, isize), bool> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().flat_map(move |(col, c)| {
                let c_start = (col * 3) as isize;
                let c_end = c_start + 3;
                let r_start = (row * 3) as isize;
                let r_end = r_start + 3;
                let center = (c_start + 1, r_start + 1);
                let walls = Rc::<[(isize, isize)]>::from(match c {
                    '-' => vec![(center.0 - 1, center.1), center, (center.0 + 1, center.1)],
                    '|' => vec![(center.0, center.1 - 1), center, (center.0, center.1 + 1)],
                    'L' => vec![(center.0, center.1 - 1), center, (center.0 + 1, center.1)],
                    'J' => vec![(center.0, center.1 - 1), center, (center.0 - 1, center.1)],
                    '7' => vec![(center.0, center.1 + 1), center, (center.0 - 1, center.1)],
                    'F' => vec![(center.0, center.1 + 1), center, (center.0 + 1, center.1)],
                    'S' => vec![
                        (center.0 - 1, center.1),
                        (center.0 + 1, center.1),
                        center,
                        (center.0, center.1 - 1),
                        (center.0, center.1 + 1),
                    ],
                    _ => vec![],
                });
                (c_start..c_end).flat_map(move |col| {
                    let walls = walls.clone();
                    (r_start..r_end).map(move |row| ((col, row), walls.contains(&(col, row))))
                })
            })
        })
        .collect()
}
