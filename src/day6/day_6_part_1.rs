use crate::day6::day_6_common::*;

fn day_6_part_1(data: &[u8]) -> i32 {
    let mut result = 0;

    let mut data = parse_data(data);
    let (mut row, mut column, mut direction) = find_guard(&data);

    while !complete(&data, row, column)  {
        let (next_row, next_column) = next_position(row, column, direction);
        if data[next_row][next_column] == b'#' {
            direction = turn_right(direction)
        } else {
            if data[row][column] != b'X' {
                result += 1;
                data[row][column] = b'X';
            }

            (row, column) = (next_row, next_column);
        }
    }

    // +1 to include the final step
    result + 1
}



mod test {
    use super::*;

    #[test]
    fn test_day_6_part_1_example() {
        let result = day_6_part_1(EXAMPLE_DATA);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_day_6_part_1() {
        let data = include_bytes!("../data/day_6_input.txt");
        let result = day_6_part_1(data);

        assert_eq!(result, 5153);
    }
}