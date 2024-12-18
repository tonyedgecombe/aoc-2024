use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn parse_data(data: &str, map_size: usize, to_take: usize) -> Vec<Vec<bool>>{
    let mut result = vec![vec![false; map_size]; map_size];

    for line in data.lines().filter(|line| !line.is_empty()).take(to_take) {
        let pair: Vec<_> = line
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        assert_eq!(pair.len(), 2);

        result[pair[0]][pair[1]] = true;
    }

    result
}

#[derive(Eq, PartialEq)]
struct Vertex {
    pos: (usize, usize),
    cost: i64,
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn count_steps(map: &Vec<Vec<bool>>) -> Option<i64> {
    let start = (0, 0);
    let end = (map.len() - 1, map.len() - 1);

    let mut visited: Vec<(usize, usize)> = Vec::new();

    let mut costs = vec![vec![i64::MAX; map.len()]; map.len()];
    costs[start.0][start.1] = 0;
    let mut heap = BinaryHeap::new();

    heap.push(Vertex {pos: start, cost: 0});

    loop {
        let Some(min) = heap.pop() else {
            return None;
        };

        let pos = min.pos;
        let cost = min.cost;

        if visited.contains(&pos) {
            continue
        }

        if pos == end {
            return Some(cost);
        }

        if cost == i64::MAX {
            return None;
        }

        visited.push(min.pos);

        if map[pos.0][pos.1] {
            continue;
        }

        if pos.0 > 0 {
            if cost + 1 < costs[pos.0 - 1][pos.1] {
                costs[pos.0 - 1][pos.1] = cost + 1;
                heap.push(Vertex { pos:(pos.0 - 1, pos.1), cost: cost + 1});
            }
        }

        if pos.0 < map.len() - 1 {
            if cost + 1 < costs[pos.0 + 1][pos.1] {
                costs[pos.0 + 1][pos.1] = cost + 1;
                heap.push(Vertex { pos:(pos.0 + 1, pos.1), cost: cost + 1});
            }
        }

        if pos.1 > 0 {
            if cost + 1 < costs[pos.0][pos.1 - 1] {
                costs[pos.0][pos.1 -1] = cost + 1;
                heap.push(Vertex { pos:(pos.0, pos.1 - 1), cost: cost + 1});
            }
        }

        if pos.1 < map.len() - 1 {
            if cost + 1 < costs[pos.0][pos.1 + 1] {
                costs[pos.0][pos.1 + 1] = cost + 1;
                heap.push(Vertex { pos:(pos.0, pos.1 + 1), cost: cost + 1});
            }
        }
    }
}

pub const EXAMPLE_DATA: &str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let map = parse_data(EXAMPLE_DATA, 7, 12);
        assert_eq!(map[3], [true, false, false, true, false, false, false]);
    }

    #[test]
    fn test_count_steps() {
        let map = parse_data(EXAMPLE_DATA, 7, 12);
        let steps = count_steps(&map).unwrap();
        assert_eq!(steps, 22);
    }
}