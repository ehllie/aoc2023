type RangeMap = (isize, isize, isize);
type ItemMap = Vec<RangeMap>;

pub fn part_one(input: &str) -> String {
    let (seeds, maps) = parse_maps(&input);
    seeds
        .iter()
        .map(|s| traverse_maps(*s, &maps))
        .min()
        .unwrap()
        .to_string()
}

pub fn part_two(input: &str) -> String {
    let (seeds, maps) = parse_maps(&input);
    let seed_ranges = seeds.windows(2).step_by(2).map(|w| (w[0], w[1]));
    let ranges = seed_ranges.collect();
    maps.iter()
        .fold(ranges, |ranges, item_map| map_ranges(ranges, item_map))
        .into_iter()
        .map(|(s, _)| s)
        .min()
        .unwrap()
        .to_string()
}

struct RangeList {
    pub ranges: Vec<(isize, isize)>,
}

impl FromIterator<(isize, isize)> for RangeList {
    fn from_iter<T: IntoIterator<Item = (isize, isize)>>(iter: T) -> Self {
        iter.into_iter().fold(Self::new(), |mut acc, range| {
            acc.insert(range);
            acc
        })
    }
}

impl IntoIterator for RangeList {
    type Item = (isize, isize);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.ranges.into_iter()
    }
}

impl RangeList {
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn cleanup_insert(&mut self, pos: usize) {
        while self.ranges.len() > pos + 1 {
            let (start, len) = self.ranges[pos];
            let next = self.ranges[pos + 1];
            if start + len >= next.0 {
                self.ranges[pos].1 = len.max(next.0 - start + next.1);
                self.ranges.remove(pos + 1);
            } else {
                break;
            }
        }
    }

    pub fn insert(&mut self, (start, len): (isize, isize)) {
        match self.ranges.binary_search_by_key(&&start, |(s, _)| s) {
            Ok(pos) => {
                self.ranges[pos].1 = len.max(self.ranges[pos].1);
                self.cleanup_insert(pos);
            }
            Err(pos) => {
                if pos > 0 {
                    let prev = self.ranges[pos - 1];
                    if prev.0 + prev.1 >= start {
                        self.ranges[pos - 1].1 = len.max(start - prev.0 + prev.1);
                        self.cleanup_insert(pos - 1);
                        return;
                    }
                }
                self.ranges.insert(pos, (start, len));
                self.cleanup_insert(pos);
            }
        }
    }

    fn cleanup_remove(&mut self, removed: &mut Self, pos: usize, end: isize) {
        while self.ranges.len() > pos {
            let next = self.ranges[pos];
            let overlap = next.1.min(end - next.0);

            if overlap > 0 {
                removed.insert((next.0, overlap));
                if overlap == next.1 {
                    self.ranges.remove(pos);
                } else {
                    self.ranges[pos] = (next.0 + overlap, next.1 - overlap)
                }
            } else {
                break;
            }
        }
    }

    pub fn remove(&mut self, (start, len): (isize, isize)) -> Self {
        let mut removed = Self::new();
        match self.ranges.binary_search_by_key(&&start, |(s, _)| s) {
            Ok(pos) => {
                let found = self.ranges[pos];
                removed.insert((start, len.min(found.1)));
                if found.1 <= len {
                    self.ranges.remove(pos);
                    self.cleanup_remove(&mut removed, pos, start + len);
                } else {
                    self.ranges[pos].1 = start - found.0;

                    let from = start + len;
                    let to = found.0 + found.1;
                    self.ranges.insert(pos + 1, (from, to - from));
                }
            }
            Err(pos) => {
                if pos > 0 {
                    let prev = self.ranges[pos - 1];
                    let prev_end = prev.0 + prev.1;

                    if prev_end >= start {
                        self.ranges.insert(pos, (start, prev_end - start));
                        self.ranges[pos - 1].1 = start - prev.0;
                    }
                }
                self.cleanup_remove(&mut removed, pos, start + len);
            }
        };
        removed
    }

    pub fn join(&mut self, other: Self) {
        for range in other.ranges {
            self.insert(range);
        }
    }
}

fn map_ranges(ranges: RangeList, item_map: &ItemMap) -> RangeList {
    let (mut unmapped, mapped) = item_map.iter().fold(
        (ranges, RangeList::new()),
        |(mut unmapped, mut mapped), (to, from, len)| {
            let to_map = unmapped.remove((*from, *len));
            let offset = to - from;
            mapped.join(to_map.into_iter().map(|(s, l)| (s + offset, l)).collect());
            (unmapped, mapped)
        },
    );
    unmapped.join(mapped);
    unmapped
}

fn traverse_maps(src: isize, maps: &Vec<ItemMap>) -> isize {
    maps.iter().fold(src, |src, m| {
        m.iter()
            .find(|(_, from, len)| (*from..(from + len)).contains(&src))
            .map(|(to, from, _)| src - from + to)
            .unwrap_or(src)
    })
}

fn parse_maps(input: &str) -> (Vec<isize>, Vec<ItemMap>) {
    let mut chunks = input.split("\n\n");
    let seeds = chunks
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .map(|s| s.split(' ').filter_map(|n| n.parse().ok()).collect())
        .unwrap_or(Vec::new());
    let maps = chunks
        .map(|chunk| {
            chunk
                .lines()
                .skip(1)
                .filter_map(|l| {
                    let mut iter = l.split(' ').filter_map(|n| n.parse().ok());
                    Some((iter.next()?, iter.next()?, iter.next()?))
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}
