use crate::day12::day_12_common::*;

pub fn count_row_top_sides(map: &Vec<Vec<u8>>, row: usize) -> usize {
    let mut sides = 0;
    let mut in_sequence = false;

    for column in 0..map[row].len() {
        if in_sequence {
            if map[row][column] == 0 || (row != 0 && map[row - 1][column] != 0) {
                in_sequence = false;
            }
        } else {
            if map[row][column] != 0 && (row == 0 || map[row - 1][column] == 0) {
                in_sequence = true;
                sides += 1;
            }
        }
    }

    sides
}

pub fn count_row_bottom_sides(map: &Vec<Vec<u8>>, row: usize) -> usize {
    let map: Vec<_> = map.iter().rev().map(|row| row.clone()).collect();
    let row = map.len() - 1 - row;
    count_row_top_sides(&map, row)
}

pub fn count_column_sides(map: &Vec<Vec<u8>>, column: usize) -> usize {
    let map = transpose_map(map);

    count_row_sides(&map, column)
}

fn transpose_map(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    (0..map[0].len())
        .map(|row| (0..map.len()).map(|col| map[col][row]).collect::<Vec<_>>())
        .collect()
}

pub fn count_row_sides(map: &Vec<Vec<u8>>, row: usize) -> usize {
    count_row_top_sides(map, row) + count_row_bottom_sides(map, row)
}

pub fn count_rows(map: &Vec<Vec<u8>>) -> usize {
    (0..map.len()).map(|row| count_row_sides(&map, row)).sum()
}

pub fn count_columns(map: &Vec<Vec<u8>>) -> usize {
    (0..map[0].len()).map(|col| count_column_sides(&map, col)).sum()
}

pub fn count_sides(map: &Vec<Vec<u8>>) -> usize {
    count_rows(map) + count_columns(map)
}

pub fn price_from_map_sides(map: &Vec<Vec<u8>>) -> usize {
    let area = area(map);
    let sides = count_sides(map);

    sides * area
}

pub fn price_from_data_sides(data: &[u8]) -> usize {
    let mut map = parse_data(data);
    let mut price = 0;

    loop {
        let Some(next_plot_position) = next_plot_position(&map) else {
            break;
        };

        let id = map[next_plot_position.0][next_plot_position.1];

        let mut plot_map = vec![vec![0; map[0].len()]; map.iter().len()];
        remove_plot(&mut map, &mut plot_map, next_plot_position, id);

        price += price_from_map_sides(&plot_map);
    }

    price
}

mod test {
    use super::*;

    // Commented out because slow
    // #[test]
    // fn test_day_12_part_2() {
    //     let price = price_from_data_sides(include_bytes!("../data/day_12_input.txt"));
    //     assert_eq!(price, 901100);
    // }

    #[test]
    fn test_price_from_map() {
        let price = price_from_data_sides(b"AAAA\nBBCD\nBBCC\nEEEC");

        assert_eq!(price, 80);
    }

    #[test]
    fn test_price_from_map_2() {
        let price = price_from_data_sides(b"AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA");

        assert_eq!(price, 368);
    }

    #[test]
    fn test_count_row_top_sides_top_row() {
        let  map = parse_data(b"AA\0AA");

        let sides = count_row_top_sides(&map, 0);

        assert_eq!(sides, 2);
    }

    #[test]
    fn test_count_row_top_sides_second_row() {
        let  map = parse_data(b"\0\0A\0\0\nAA\0AA");

        let sides = count_row_top_sides(&map, 1);

        assert_eq!(sides, 2);
    }

    #[test]
    fn test_count_row_bottom_sides_top_row() {
        let  map = parse_data(b"AA\0AA");

        let sides = count_row_bottom_sides(&map, 0);

        assert_eq!(sides, 2);
    }

    #[test]
    fn test_count_row_bottom_sides_top_row_split() {
        let  map = parse_data(b"AAAAA\nAA\0AA");

        let sides = count_row_bottom_sides(&map, 1);

        assert_eq!(sides, 2);
    }

    #[test]
    fn test_count_row_bottom_sides_bottom_row() {
        let  map = parse_data(b"\0\0A\0\0\nAA\0AA");

        let sides = count_row_bottom_sides(&map, 1);

        assert_eq!(sides, 2);
    }

    #[test]
    fn test_count_row_sides() {
        let map = parse_data(b"\0\0A\0\0\nAAAAA");

        let sides = count_row_sides(&map, 1);

        assert_eq!(sides, 3);
    }

    #[test]
    fn test_invert_map() {
        let map = parse_data(b"12\n34\n");
        let inverted  = transpose_map(&map);

        assert_eq!(inverted, [b"13", b"24"]);
    }

    #[test]
    fn test_count_column_sides() {
        let map = parse_data(b"\0A\n\0A\nAA\n\0A\n\0A");

        let sides = count_column_sides(&map, 1);

        assert_eq!(sides, 3);
    }

    #[test]
    fn test_count_rows() {
        let map = parse_data(b"\0\0A\0\0\nAAAAA");

        let sides = count_rows(&map);

        assert_eq!(sides, 4);
    }

    #[test]
    fn test_count_columns() {
        let map = parse_data(b"\0A\n\0A\nAA\n\0A\n\0A");

        let sides = count_columns(&map);

        assert_eq!(sides, 4);
    }
}