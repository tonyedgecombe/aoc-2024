pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_byte(c: u8) -> Option<Direction> {
        match c {
            b'^' => Some(Direction::Up),
            b'v' => Some(Direction::Down),
            b'<' => Some(Direction::Left),
            b'>' => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            Direction::Up => b'^',
            Direction::Down => b'v',
            Direction::Left => b'<',
            Direction::Right => b'>',
        }
    }
}

pub fn parse_moves(data: &[u8]) -> Vec<Direction> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .skip_while(|line| line[0] == b'#')
        .map(|line| line
            .iter()
            .filter_map(|&c| Direction::from_byte(c))
            .collect::<Vec<_>>())
        .flatten()
        .collect()
}

pub fn parse_map(data: &[u8]) -> Vec<Vec<u8>> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .filter(|line| line[0] == b'#')
        .map(|line| Vec::from(line))
        .collect()
}

pub fn find_robot(map: &Vec<Vec<u8>>) -> (usize, usize) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == b'@' {
                return (row, col)
            }
        }
    }

    panic!("No robot ]-<");
}


pub fn sum_of_coordinates_by_char(map: &Vec<Vec<u8>>, c: u8) -> usize {
    let mut sum = 0;

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == c {
                sum += row * 100 + col;
            }
        }
    }

    sum
}

pub const SMALL_EXAMPLE: &[u8] = b"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv
<v>>v<<";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_moves() {
        let moves = parse_moves(SMALL_EXAMPLE);

        assert_eq!(moves.iter().map(|d| d.to_byte()).collect::<Vec<u8>>(), b"<^^>>>vv<v>>v<<");
    }

    #[test]
    fn test_parse_map() {
        let map = parse_map(SMALL_EXAMPLE);

        assert_eq!(map.len(), 8);
        assert_eq!(map[2], b"##@.O..#");
    }

    #[test]
    fn test_find_robot() {
        let map = parse_map(SMALL_EXAMPLE);

        let robot = find_robot(&map);

        assert_eq!(robot, (2, 2));
    }



}