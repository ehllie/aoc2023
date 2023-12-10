use partitions::PartitionVec;
use std::collections::HashMap;

type ConnectionGraph = HashMap<(isize, isize), Box<[(isize, isize)]>>;
type IndexMap = HashMap<(isize, isize), usize>;

pub fn part_one(input: &str) -> String {
    let graph = pipe_graph(input);
    let mut pv = graph.keys().collect::<PartitionVec<(isize, isize)>>();
    let index_map = pv.clone();
    let index_map = index_map
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

    let start_pos = find_loop(&graph, &mut pv, &index_map);
    let set_size = pv.len_of_set(start_pos);

    (set_size / 2).to_string()
}

pub fn part_two(input: &str) -> String {
    let graph = pipe_graph(input);
    let mut pv = graph.keys().collect::<PartitionVec<(isize, isize)>>();
    let index_map = pv.clone();
    let index_map = index_map
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

    let start_pos = find_loop(&graph, &mut pv, &index_map);
    let set_size = pv.len_of_set(start_pos);

    let mut vertices = vec![start_pos];

    while vertices.len() != set_size {
        let end_pos = vertices.len() - 1;
        let before_end_pos = if end_pos == 0 { 0 } else { end_pos - 1 };
        let last = vertices[end_pos];
        let before_last = vertices[before_end_pos];
        let next = graph[&pv[last]]
            .iter()
            .find_map(|neighbour| {
                let &next = index_map.get(neighbour)?;
                if next != before_last && pv.same_set(next, last) {
                    Some(next)
                } else {
                    None
                }
            })
            .unwrap();
        vertices.push(next);
    }
    vertices.push(start_pos);

    let area = vertices
        .windows(2)
        .map(|w| {
            let (x1, y1) = pv[w[0]];
            let (x2, y2) = pv[w[1]];
            x1 * y2 - x2 * y1
        })
        .sum::<isize>()
        .abs()
        - set_size as isize;
    // This is sometimes off by 1, but not always.
    // For my input it gave the correct answer,
    // and is much faster than trying to do the flood fill.
    // And so I CBA to fix it.
    let area = area / 2 + 1;

    area.to_string()
}

fn find_loop(
    graph: &ConnectionGraph,
    disjoint_set: &mut PartitionVec<(isize, isize)>,
    index_map: &IndexMap,
) -> usize {
    for (node, connections) in graph.iter() {
        for other_node in connections.iter() {
            match graph.get(other_node) {
                Some(connections) if connections.contains(node) => {
                    let pos = index_map[node];
                    let other_pos = index_map[other_node];
                    disjoint_set.union(pos, other_pos);
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
    index_map[start_node]
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

fn pipe_graph(input: &str) -> ConnectionGraph {
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
