use crate::day15::day_15_common::*;

pub fn move_robot_one_step(map: &mut Vec<Vec<u8>>, direction: &Direction) {
    let robot = find_robot(map);

    move_piece(map, robot, direction);
}

pub fn move_robot(map: &mut Vec<Vec<u8>>, directions: &Vec<Direction>) {
    for direction in directions {
        move_robot_one_step(map, direction);
    }
}

pub fn move_piece(map: &mut Vec<Vec<u8>>, (row, col): (usize, usize), direction: &Direction) -> bool {
    let (new_row, new_col) = match direction {
        Direction::Up => (row - 1, col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
    };


    match map[new_row][new_col] {
        b'#' => {
            false
        }
        b'.' => {
            let tmp = map[new_row][new_col];
            map[new_row][new_col] = map[row][col];
            map[row][col] = tmp;

            true
        }
        _ => {
            if move_piece(map, (new_row, new_col), direction) {
                move_piece(map, (row, col), direction)
            } else {
                false
            }
        }
    }
}

fn sum_of_coordinates(map: &Vec<Vec<u8>>) -> usize {
    sum_of_coordinates_by_char(map, b'O')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_robot_one_step() {
        let mut map = parse_map(b"#...@..#");
        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(&map[0], b"#..@...#");
    }

    #[test]
    fn test_move_robot_into_wall() {
        let mut map = parse_map(b"#@.....#");

        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(&map[0], b"#@.....#");
    }

    #[test]
    fn test_move_box() {
        let mut map = parse_map(b"#..O@..#");
        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(&map[0], b"#.O@...#");
    }

    #[test]
    fn test_move_box_into_wall() {
        let mut map = parse_map(b"#O@....#");
        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(&map[0], b"#O@....#");
    }

    #[test]
    fn test_move_boxes_into_wall() {
        let mut map = parse_map(b"#OO@...#");
        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(&map[0], b"#OO@...#");
    }

    #[test]
    fn test_move_boxes() {
        let mut map = parse_map(b"#.OO@..#");
        move_robot_one_step(&mut map, &Direction::Left);

        assert_eq!(&map[0], b"#OO@...#");
    }

    #[test]
    fn test_move_robot() {
        let final_map = parse_map(b"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########");

        let mut map = parse_map(SMALL_EXAMPLE);
        let directions = parse_moves(SMALL_EXAMPLE);

        move_robot(&mut map, &directions);

        assert_eq!(map, final_map);
    }

    #[test]
    fn test_sum_of_coordinates_small() {
        let mut map = parse_map(SMALL_EXAMPLE);
        let directions = parse_moves(SMALL_EXAMPLE);
        move_robot(&mut map, &directions);

        let result = sum_of_coordinates(&map);

        assert_eq!(result, 2028);
    }

    #[test]
    fn test_sum_of_coordinates() {
        let data = include_bytes!("../data/day_15_input.txt");

        let mut map = parse_map(data);
        let directions = parse_moves(data);
        move_robot(&mut map, &directions);

        let result = sum_of_coordinates(&map);

        assert_eq!(result, 1514333);
    }
}
