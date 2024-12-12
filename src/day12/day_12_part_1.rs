use crate::day12::day_12_common::*;

pub fn perimeter(map: &Vec<Vec<u8>>) -> usize {
    let mut perimeter = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 0 {
                continue;
            }

            // Left
            if row == 0 || map[row - 1][col] == 0 {
                perimeter += 1;
            }

            // Top
            if col == 0 || map[row][col - 1] == 0 {
                perimeter += 1;
            }

            // Right
            if row == map.len() - 1 || map[row + 1][col] == 0 {
                perimeter += 1;
            }

            // Bottom
            if col == map[0].len() - 1 || map[row][col + 1] == 0 {
                perimeter += 1;
            }
        }
    }

    perimeter / 1
}

pub fn price_from_map(map: &Vec<Vec<u8>>) -> usize {
    let area = area(map);
    let perimeter = perimeter(map);

    perimeter * area
}

pub fn price_from_data(data: &[u8]) -> usize {
    let mut map = parse_data(data);
    let mut price = 0;

    loop {
        let Some(next_plot_position) = next_plot_position(&map) else {
            break;
        };

        let id = map[next_plot_position.0][next_plot_position.1];

        let mut plot_map = vec![vec![0; map[0].len()]; map.iter().len()];
        remove_plot(&mut map, &mut plot_map, next_plot_position, id);

        price += price_from_map(&plot_map);
    }

    price
}

mod test {
    use crate::day12::day_12_common::EXAMPLE_DATA;
    use super::*;

    #[test]
    fn test_day_12_part_1() {
        let price = price_from_data(include_bytes!("../data/day_12_input.txt"));

        assert_eq!(price, 1473276);
    }


    #[test]
    fn test_price_from_map() {
        let price = price_from_data(EXAMPLE_DATA);

        assert_eq!(price, 1930);
    }

    #[test]
    fn test_perimeter() {
        let mut map = parse_data(b"AAAA\nBBCD\nBBCC\nEEEC");
        let mut plot_map = vec![vec![0; map[0].len()]; map.iter().len()];

        remove_plot(&mut map, &mut plot_map, (0, 0), b'A');
        let perimeter = perimeter(&plot_map);
        assert_eq!(perimeter, 10);
    }

    #[test]
    fn test_price() {
        let mut map = parse_data(b"AAAA\nBBCD\nBBCC\nEEEC");
        let mut plot_map = vec![vec![0; 4]; 4];

        remove_plot(&mut map, &mut plot_map, (0, 0), b'A');

        let price = price_from_map(&plot_map);
        assert_eq!(price, 40);
    }
}