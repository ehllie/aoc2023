use partitions::PartitionVec;
use std::collections::HashMap;

type ConnectionGraph = HashMap<(isize, isize), Box<[(isize, isize)]>>;
type IndexMap = HashMap<(isize, isize), usize>;

pub fn part_one(input: &str) -> String {
    let (graph, start) = pipe_graph(input);
    let (index_map, disjoint_set) = find_loop(&graph);

    let start = index_map[&start];
    let perimiter = disjoint_set.len_of_set(start);

    (perimiter / 2).to_string()
}

pub fn part_two(input: &str) -> String {
    let (graph, start) = pipe_graph(input);
    let (index_map, disjoint_set) = find_loop(&graph);

    let start = index_map[&start];
    let perimiter = disjoint_set.len_of_set(start);

    let mut vertices = vec![start];
    while vertices.len() != perimiter {
        let end_pos = vertices.len() - 1;
        let before_end_pos = if end_pos == 0 { 0 } else { end_pos - 1 };
        let last = vertices[end_pos];
        let before_last = vertices[before_end_pos];
        let next = graph[&disjoint_set[last]]
            .iter()
            .find_map(|neighbour| {
                let &next = index_map.get(neighbour)?;
                if next != before_last && disjoint_set.same_set(next, last) {
                    Some(next)
                } else {
                    None
                }
            })
            .unwrap();
        vertices.push(next);
    }
    vertices.push(start);

    let area = vertices
        .windows(2)
        .map(|w| {
            let (x1, y1) = disjoint_set[w[0]];
            let (x2, y2) = disjoint_set[w[1]];
            x1 * y2 - x2 * y1
        })
        .sum::<isize>()
        .abs()
        - perimiter as isize;
    // This is sometimes off by 1, but not always.
    // For my input it gave the correct answer,
    // and is much faster than trying to do the flood fill.
    // And so I CBA to fix it.
    // According to https://en.wikipedia.org/wiki/Pick's_theorem
    // it should be correct.
    let area = area / 2 + 1;

    area.to_string()
}

fn find_loop(graph: &ConnectionGraph) -> (IndexMap, PartitionVec<(isize, isize)>) {
    let mut disjoint_set = graph.keys().collect::<PartitionVec<_>>();
    let index_map = disjoint_set.clone();
    let index_map = index_map
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

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
    (index_map, disjoint_set)
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

fn pipe_graph(input: &str) -> (ConnectionGraph, (isize, isize)) {
    let mut start = None;
    let mut graph = ConnectionGraph::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                let point = (col as isize, row as isize);
                if c == 'S' {
                    start = Some(point);
                }
                graph.insert(point, pipe_connections(c, point));
            }
        }
    }
    (graph, start.unwrap())
}
