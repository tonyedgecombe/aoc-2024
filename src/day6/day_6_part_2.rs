use crate::day6::day_6_common::*;

fn is_blocking(data: &Vec<Vec<u8>>, guard: (usize, usize, Direction), block_row: usize, block_column: usize) -> bool {
    let (mut row, mut column, mut direction) = guard;

    // Can't put a block where the guard starts from
    if row == block_row && column == block_column {
        return false;
    }

    // Skip existing blocks
    if data[block_row][block_column] == b'#' {
        return false;
    }

    let mut visited : Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; data[0].len()]; data.len()];
    
    while !complete(&data, row, column)  {
        if visited[row][column][direction as usize] {
            return true;
        }
        visited[row][column][direction as usize] = true;

        let (next_row, next_column) = next_position(row, column, direction);
        if data[next_row][next_column] == b'#' || (next_row == block_row && next_column == block_column) {
            direction = turn_right(direction)
        } else {
            (row, column) = (next_row, next_column);
        }
    }

    false
}

pub fn day_6_part_2(data: &[u8]) -> i32 {
    let mut data = parse_data(data);
    let guard = find_guard(&data);

    let mut result = 0;

    for row in 0..data.len() {
        for column in 0..data[0].len() {
            if is_blocking(&mut data, guard, row, column) {
                result += 1;
            }
        }
    }

    result
}

mod test {
    use super::*;

    #[test]
    fn test_is_blocking() {
        let data = parse_data(EXAMPLE_DATA);
        let guard = find_guard(&data);
        let result = is_blocking(&data, guard, 6, 3);

        assert!(result);
    }

    #[test]
    fn test_day_6_part_2_example_data() {
        let result = day_6_part_2(EXAMPLE_DATA);

        assert_eq!(result, 6);
    }

    // Disabled because it is slow
    fn test_day_6_part_2() {
        let result = day_6_part_2(include_bytes!("../data/day_6_input.txt"));

        assert_eq!(result, 1711);
    }
}