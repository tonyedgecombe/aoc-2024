use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    pub fn from_i64(i: i64) -> Option<Direction> {
        match i {
            0 => Some(Direction::North),
            1 => Some(Direction::East),
            2 => Some(Direction::South),
            3 => Some(Direction::West),
            _ => None,
        }
    }
}

pub fn parse_data(data: &[u8]) -> Vec<Vec<u8>> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Vec::from(line))
        .collect()
}


pub fn find_char(map: &Vec<Vec<u8>>, c: u8) -> (usize, usize) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == c {
                return (row, col)
            }
        }
    }

    panic!()
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Vertex {
    pos: (usize, usize),
    direction: Direction,
}

#[derive(Eq, PartialEq)]
struct VertexWithCost {
    pos: (usize, usize),
    direction: Direction,
    cost: i64,
}

impl Ord for VertexWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| (self.direction as usize).cmp(&(other.direction as usize)))
    }
}

impl PartialOrd for VertexWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn count_tiles_from_end(costs: &mut Vec<Vec<Vec<i64>>>, end: (usize, usize), min: i64) -> i64 {
    // Discard the non solutions from the end point
    for d in 0..4 {
        if costs[end.0][end.1][d] != min {
            costs[end.0][end.1][d] = i64::MAX;
        }
    }
    let mut hash_set = HashSet::new();

    find_tiles(costs, end, min, &mut hash_set);

    hash_set.len() as i64
}

fn find_tiles(costs: &Vec<Vec<Vec<i64>>>, (row, col): (usize, usize), next_cost: i64, hash_set: &mut HashSet<(usize, usize)>) {
    hash_set.insert((row, col));

    for d in 0..4 {
        let next_direction = Direction::from_i64(d as i64).unwrap();

        let position = match next_direction {
            Direction::North => (row + 1, col),
            Direction::East => (row, col - 1),
            Direction::South => (row - 1, col),
            Direction::West => (row, col + 1),
        };

        for pre_dir in 0..4 {
            let cost = costs[position.0][position.1][pre_dir];

            if cost == i64::MAX {
                continue;
            }

            let cost_offset = match i64::abs(d as i64 - pre_dir as i64) {
                0 => 1,
                1 | 3 => 1001,
                2 => 2001,
                _ => panic!(),
            };

            if cost + cost_offset == next_cost {
                find_tiles(costs, position, cost, hash_set);
            }
        }
    }
}

pub fn calculate_cost(data: &[u8]) -> Option<(i64, i64)> {
    let map = parse_data(data);
    let start = find_char(&map, b'S');
    let end = find_char(&map, b'E');

    let mut visited: Vec<Vertex> = Vec::new();

    let mut costs = vec![vec![vec![i64::MAX; 4]; map[0].len()]; map.len()];
    let mut heap = BinaryHeap::new();

    heap.push(VertexWithCost {pos: start, direction:Direction::East, cost: 0});
    costs[start.0][start.1][Direction::East as usize] = 0;

    loop {
        let Some(min) = heap.pop() else {
            return None;
        };

        if visited.contains(&Vertex {pos: min.pos, direction: min.direction}) {
            continue;
        }

        if min.pos == end {
            let tiles = count_tiles_from_end(&mut costs, end, min.cost);
            return Some((min.cost, tiles));
        };

        if min.cost == i64::MAX {
            panic!();
        }

        visited.push(Vertex{ pos: min.pos, direction: min.direction});

        for i in 0..4 {
            let direction = Direction::from_i64(i).unwrap();

            let cost = min.cost + match i64::abs(min.direction as i64 - i) {
                0 => 1,
                1 | 3 => 1001,
                2 => 2001,
                _ => panic!()
            };

            let next = match direction {
                Direction::North => (min.pos.0 - 1, min.pos.1),
                Direction::East => (min.pos.0, min.pos.1 + 1),
                Direction::South => (min.pos.0 + 1, min.pos.1),
                Direction::West => (min.pos.0, min.pos.1 - 1),
            };

            if map[next.0][next.1] == b'#' {
                continue;
            }

            if cost < costs[next.0][next.1][direction as usize] {
                costs[next.0][next.1][direction as usize] = cost;
                heap.push(VertexWithCost {
                    pos: next,
                    direction,
                    cost,
                });
            }
        }
    }
}

pub const EXAMPLE_DATA: &[u8] = b"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

const EXAMPLE_DATA_2: &[u8] = b"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

const EXAMPLE_DATA_3: &[u8] = b"###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################";

pub const EXAMPLE_DATA_5: &[u8] = b"##########
#.......E#
#.##.#####
#..#.....#
##.#####.#
#S.......#
##########";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let map = parse_data(EXAMPLE_DATA);
        assert_eq!(map.len(), 15);
        assert_eq!(map[1], b"#.......#....E#");
    }

    #[test]
    fn test_calculate_cost_no_rotation_one_step() {
        let data = b"####\n#SE#\n####";
        let (cost, _tiles) = calculate_cost(data).unwrap();

        assert_eq!(cost, 1);
    }

    #[test]
    fn test_calculate_cost_no_rotation_two_steps() {
        let data = b"#####\n#S.E#\n#####";
        let (cost, _tiles) = calculate_cost(data).unwrap();

        assert_eq!(cost, 2);
    }

    #[test]
    fn test_calculate_cost_two_rotations_and_two_steps() {
        let data = b"#####\n#E.S#\n#####";
        let (cost, _tiles) = calculate_cost(data).unwrap();

        assert_eq!(cost, 2002);
    }

    #[test]
    fn test_calculate_cost_two_routes() {
        let data = b"####\n#S.#\n#.E#\n####";
        let (cost, _tiles) = calculate_cost(data).unwrap();

        assert_eq!(cost, 1002);
    }

    #[test]
    fn test_calculate_cost() {
        let (cost, tiles) = calculate_cost(EXAMPLE_DATA).unwrap();
        assert_eq!(cost, 7036);
        assert_eq!(tiles, 45);
    }

    #[test]
    fn test_calculate_cost2() {
        let (cost, tiles) = calculate_cost(EXAMPLE_DATA_2).unwrap();
        assert_eq!(cost, 11048);
        assert_eq!(tiles, 64);
    }

    #[test]
    fn test_calculate_cost3() {
        let (cost, _tiles) = calculate_cost(EXAMPLE_DATA_3).unwrap();
        assert_eq!(cost, 21148);
    }

    #[test]
    fn test_calculate_cost5() {
        let (cost, tiles) = calculate_cost(EXAMPLE_DATA_5).unwrap();
        assert_eq!(cost, 4013);
        assert_eq!(tiles, 14)
    }
}