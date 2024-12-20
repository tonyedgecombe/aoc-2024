pub fn parse_data(data: &[u8]) -> Vec<Vec<u8>> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| Vec::from(line))
        .collect()
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction { North, East, South, West }


pub fn find_char(map: &Vec<Vec<u8>>, char: u8) -> Option<(i64, i64)> {
    for row in 0..map.len() {
        for col in 0..map.len() {
            if map[row][col] == char {
                return Some((row as i64, col as i64))
            }
        }
    }

    None
}


fn count_steps(map: &Vec<Vec<u8>>) -> usize {
    get_steps(map).iter().count() - 1
}

pub fn get_steps(map: &Vec<Vec<u8>>) -> Vec<(i64, i64)> {
    let end = find_char(map, b'E').unwrap();
    let mut pos = find_char(map, b'S').unwrap();

    let mut visited = Vec::new();

    while pos != end {
        visited.push(pos);

        let Some(next_direction) = next_direction(map, &mut pos, &visited) else {
            panic!();
        };

        pos = next_pos_from_direction(pos, next_direction);
    }

    visited.push(pos);

    visited
}

fn next_direction(map: &Vec<Vec<u8>>, pos: &(i64, i64), visited: &Vec<(i64, i64)>) -> Option<Direction> {
    for direction in [Direction::North, Direction::East, Direction::South, Direction::West] {
        let next_pos = match direction {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::West => (pos.0, pos.1 - 1),
        };

        if map[next_pos.0 as usize][next_pos.1 as usize] == b'#' {
            continue;
        }

        if visited.contains(&next_pos) {
            continue;
        }

        return Some(direction);
    }

    None
}

fn next_pos_from_direction((row, column): (i64, i64), direction:Direction) -> (i64, i64) {
    match direction {
        Direction::North => (row - 1, column),
        Direction::East => (row, column + 1),
        Direction::South => (row + 1, column),
        Direction::West => (row, column - 1),
    }
}

pub const EXAMPLE_DATA: &[u8] = b"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let map = parse_data(EXAMPLE_DATA);

        assert_eq!(map.len(), 15);
        assert_eq!(map[0].len(), 15);
        assert_eq!(map[1], b"#...#...#.....#");
    }

    #[test]
    fn test_count_steps() {
        let map = parse_data(EXAMPLE_DATA);
        let steps = count_steps(&map);

        assert_eq!(steps, 84);
    }
}