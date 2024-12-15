use crate::day15::day_15_common::*;

fn double_char(c: &u8) -> Vec<u8> {
    match *c {
        b'#' => vec![b'#', b'#'],
        b'O' => vec![b'[', b']'],
        b'.' => vec![b'.', b'.'],
        b'@' => vec![b'@', b'.'],
        _ => panic!("Oh no"),
    }
}

fn double_row(row: Vec<u8>) -> Vec<u8> {
    row
        .iter()
        .map(double_char)
        .flatten()
        .collect()
}

fn double_map(map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    map
        .into_iter()
        .map(double_row)
        .collect()
}

pub fn move_robot_one_step(map: &mut Vec<Vec<u8>>, direction: &Direction) {
    let robot = find_robot(map);

    if space_available(map, robot, direction) {
        move_piece(map, robot, direction);
    }
}

pub fn move_robot(map: &mut Vec<Vec<u8>>, directions: &Vec<Direction>) {
    for direction in directions {
        move_robot_one_step(map, direction);
    }
}

fn move_piece(map: &mut Vec<Vec<u8>>, pos: (usize, usize), direction: &Direction) {
    match direction {
        Direction::Up => move_piece_up(map, pos),
        Direction::Down => move_piece_down(map, pos),
        Direction::Left => move_piece_left(map, pos),
        Direction::Right => move_piece_right(map, pos),
    }
}

fn move_piece_up(map: &mut Vec<Vec<u8>>, (row, col): (usize, usize)) {
    let (new_row, new_col) = (row - 1, col);

    match map[new_row][new_col] {
        b'.' => {},

        b'[' => {
            move_piece_up(map, (new_row, new_col));
            move_piece_up(map, (new_row, new_col + 1));
        },

        b']' => {
            move_piece_up(map, (new_row, new_col));
            move_piece_up(map, (new_row, new_col - 1));
        },

        _ => panic!(),
    }

    let tmp = map[new_row][new_col];
    map[new_row][new_col] = map[row][col];
    map[row][col] = tmp;
}

fn move_piece_down(map: &mut Vec<Vec<u8>>, (row, col): (usize, usize)) {
    let (new_row, new_col) = (row + 1, col);

    match map[new_row][new_col] {
        b'.' => {},

        b'[' => {
            move_piece_down(map, (new_row, new_col));
            move_piece_down(map, (new_row, new_col + 1));
        },

        b']' => {
            move_piece_down(map, (new_row, new_col));
            move_piece_down(map, (new_row, new_col - 1));
        },

        _ => panic!(),
    }

    let tmp = map[new_row][new_col];
    map[new_row][new_col] = map[row][col];
    map[row][col] = tmp;
}

fn move_piece_left(map: &mut Vec<Vec<u8>>, (row, col): (usize, usize)) {
    let (new_row, new_col) = (row, col - 1);

    match map[new_row][new_col] {
        b'#' => panic!("Bollocks"),
        b'.' => {},

        _ => {
            move_piece_left(map, (new_row, new_col));
        }
    }

    map[new_row][new_col] = map[row][col];
    map[row][col] = b'.';
}

fn move_piece_right(map: &mut Vec<Vec<u8>>, (row, col): (usize, usize)) {
    let (new_row, new_col) = (row, col + 1);

    match map[new_row][new_col] {
        b'#' => panic!("Bollocks"),
        b'.' => {},

        _ => {
            move_piece_right(map, (new_row, new_col));
        }
    }

    map[new_row][new_col] = map[row][col];
    map[row][col] = b'.';
}

fn space_available(map: &Vec<Vec<u8>>, (row, col): (usize, usize), direction: &Direction) -> bool {
    let piece = map[row][col];
    if piece == b'.' {
        return true;
    } else if piece == b'#' {
        return false;
    }

    match direction {
        Direction::Up => space_available_up(map, (row, col)),
        Direction::Down => space_available_down(map, (row, col)),
        Direction::Left => space_available(map, (row, col - 1), direction),
        Direction::Right => space_available(map, (row, col + 1), direction),
    }
}

fn space_available_up(map: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
    match map[row][col] {
        b'.' => true,
        b'#' => false,
        b'@' => space_available_up(map, (row - 1, col)),
        b'[' => space_available_up(map, (row - 1, col)) && space_available_up(map, (row - 1, col + 1)),
        b']' => space_available_up(map, (row - 1, col)) && space_available_up(map, (row - 1, col - 1)),
        _ => panic!("Oh fuck")
    }
}

fn space_available_down(map: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
    match map[row][col] {
        b'.' => true,
        b'#' => false,
        b'@' => space_available_down(map, (row + 1, col)),
        b'[' => space_available_down(map, (row + 1, col)) && space_available_down(map, (row + 1, col + 1)),
        b']' => space_available_down(map, (row + 1, col)) && space_available_down(map, (row + 1, col - 1)),
        _ => panic!("Oh fuck")
    }
}

pub fn sum_of_coordinates(map: &Vec<Vec<u8>>) -> usize {
    sum_of_coordinates_by_char(map, b'[')
}

#[cfg(test)]
mod test {
    use super::*;

    const LARGE_EXAMPLE: &[u8] = b"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const DOUBLE_LARGE_EXAMPLE: &[u8] = b"####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################";

    #[test]
    fn test_double_map() {
        let map = parse_map(LARGE_EXAMPLE);

        let map = double_map(map);

        let expected_map = parse_map(DOUBLE_LARGE_EXAMPLE);
        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_space_available_left() {
        let map = parse_map(b"#...@..#");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Left));

        let map = parse_map(b"#@.....#");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Left));

        let map = parse_map(b"#.[]@..#");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Left));

        let map = parse_map(b"#[]@...#");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Left));
    }

    #[test]
    fn test_move_piece_left() {
        let mut map = parse_map(b"#...@..#");
        let expected = parse_map(b"#..@...#");

        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(map, expected);

        let mut map = parse_map(b"#@.....#");
        let expected = parse_map(b"#@.....#");

        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(map, expected);


        let mut map = parse_map(b"#.[]@..#");
        let expected = parse_map(b"#[]@...#");

        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(map, expected);


        let mut map = parse_map(b"#[]@...#");
        let expected = parse_map(b"#[]@...#");

        move_robot_one_step(&mut map, &Direction::Left);
        assert_eq!(map, expected);
    }

    #[test]
    fn test_space_available_right() {
        let map = parse_map(b"#...@..#");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Right));

        let map = parse_map(b"#.....@#");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Right));

        let map = parse_map(b"#..@[]..#");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Right));

        let map = parse_map(b"#...@[]#");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Right));
    }

    #[test]
    fn test_move_piece_right() {
        let mut map = parse_map(b"#...@..#");
        let expected = parse_map(b"#....@.#");

        move_robot_one_step(&mut map, &Direction::Right);

        assert_eq!(map, expected);


        let mut map = parse_map(b"#.....@#");
        let expected = parse_map(b"#.....@#");

        move_robot_one_step(&mut map, &Direction::Right);

        assert_eq!(map, expected);


        let mut map = parse_map(b"#..@[]..#");
        let expected = parse_map(b"#...@[].#");

        move_robot_one_step(&mut map, &Direction::Right);

        assert_eq!(map, expected);


        let mut map = parse_map(b"#...@[]#");
        let expected = parse_map(b"#...@[]#");

        move_robot_one_step(&mut map, &Direction::Right);

        assert_eq!(map, expected);
    }

    #[test]
    fn test_space_available_up() {
        let map = parse_map(b"#.\n#@");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Up));

        let map = parse_map(b"##\n#@");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Up));

        let map = parse_map(b"#..\n#[]\n#@.");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Up));

        let map = parse_map(b"#..\n#[]\n#.@");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Up));

        let map = parse_map(b"##.\n#[]\n#@.");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Up));

        let map = parse_map(b"#.#\n#[]\n#@.");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Up));

        let map = parse_map(b"#.#\n#[]\n#.@");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Up));

        let map = parse_map(b"##.\n#[]\n#.@");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Up));
    }

    #[test]
    fn test_move_piece_up() {
        let mut map = parse_map(b"#.\n#@");
        let expected = parse_map(b"#@\n#.");

        move_robot_one_step(&mut map, &Direction::Up);

        assert_eq!(map, expected);

        //////////////

        let mut map = parse_map(b"#..\n#[]\n#@.");
        let expected = parse_map(b"#[]\n#@.\n#..");

        move_robot_one_step(&mut map, &Direction::Up);

        assert_eq!(map, expected);

        /////////////

        let mut map = parse_map(b"#..\n#[]\n#.@");
        let expected = parse_map(b"#[]\n#.@\n#..");

        move_robot_one_step(&mut map, &Direction::Up);

        assert_eq!(map, expected);
    }

    #[test]
    fn test_space_available_down() {
        let map = parse_map(b"#@\n#.");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Down));

        let map = parse_map(b"#@\n##");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Down));

        let map = parse_map(b"#@.\n#[]\n#..");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Down));

        let map = parse_map(b"#.@\n#[]\n#..");
        let robot = find_robot(&map);
        assert!(space_available(&map, robot, &Direction::Down));

        let map = parse_map(b"#@.\n#[]\n##.");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Down));

        let map = parse_map(b"#@.\n#[]\n#.#");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Down));

        let map = parse_map(b"#.@\n#[]\n#.#");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Down));

        let map = parse_map(b"#.@\n#[]\n##.");
        let robot = find_robot(&map);
        assert!(!space_available(&map, robot, &Direction::Down));
    }

    #[test]
    fn test_move_piece_down() {
        let mut map = parse_map(b"#@\n#.");
        let expected = parse_map(b"#.\n#@\n");

        move_robot_one_step(&mut map, &Direction::Down);

        assert_eq!(map, expected);

        /////////////

        let mut map = parse_map(b"#@.\n#[]\n#..");
        let expected = parse_map(b"#..\n#@.\n#[]");

        move_robot_one_step(&mut map, &Direction::Down);

        assert_eq!(map, expected);

        /////////////

        let mut map = parse_map(b"#.@\n#[]\n#..");
        let expected = parse_map(b"#..\n#.@\n#[]");
        move_robot_one_step(&mut map, &Direction::Down);

        assert_eq!(map, expected);
    }

    #[test]
    fn day_15_part_2() {
        let data = include_bytes!("../data/day_15_input.txt");

        let map = parse_map(data);
        let mut map = double_map(map);

        let directions = parse_moves(data);
        move_robot(&mut map, &directions);

        let result = sum_of_coordinates(&map);

        assert_eq!(result, 1528453);
    }
}