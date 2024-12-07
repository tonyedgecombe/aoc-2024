#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_data(data: &[u8]) -> Vec<Vec<u8>> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| !String::from_utf8(line.to_vec()).unwrap().is_empty())
        .map(|line| line.to_vec()).collect()
}

pub fn find_guard(data: &Vec<Vec<u8>>) -> (usize, usize, Direction) {
    for row in 0..data[0].len() {
        for column in 0..data.len() {
            match data[row][column] {
                b'^' => return (row, column, Direction::Up),
                b'v' => return (row, column, Direction::Down),
                b'>' => return (row, column, Direction::Right),
                b'<' => return (row, column, Direction::Left),
                _ => continue,
            }
        }
    }

    panic!()
}

pub fn next_position(row: usize, column: usize, direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (row - 1, column),
        Direction::Down => (row + 1, column),
        Direction::Left => (row, column - 1),
        Direction::Right => (row, column + 1),
    }
}

pub fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
    }
}

pub fn complete(data: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    row == 0 || row == data.len() - 1 || column == 0 || column == data[0].len() - 1
}

pub const EXAMPLE_DATA: &[u8] = b"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

mod test {
    use super::*;



    #[test]
    fn test_parse_data() {
        let data = parse_data(EXAMPLE_DATA);

        assert_eq!(data.len(), 10);
        assert_eq!(data[0].len(), 10);
    }

    #[test]
    fn test_find_guard() {
        let data = parse_data(EXAMPLE_DATA);
        let (row, column, direction) = find_guard(&data);

        assert_eq!(row, 6);
        assert_eq!(column, 4);
        assert_eq!(direction, Direction::Up);
    }
}