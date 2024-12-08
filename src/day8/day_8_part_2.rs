use std::collections::HashSet;
use crate::day8::day_8_common::*;

fn get_antinodes_all(antennas: &Vec<Antenna>, width: i32, height: i32) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i >= j {
                continue;
            }

            let first = &antennas[i];
            let second = &antennas[j];

            if first.typ != second.typ {
                continue;
            }

            let (d_row, d_col) = (first.row - second.row, first.col - second.col);

            let mut antinode = (first.row, first.col);
            while within_bounds(antinode, width, height) {
                result.insert(antinode);

                antinode = (antinode.0 + d_row, antinode.1 + d_col);
            }

            let mut antinode = (second.row, second.col);
            while within_bounds(antinode, width, height) {
                result.insert(antinode);

                antinode = (antinode.0 - d_row, antinode.1 - d_col);
            }
        }
    }

    result
}

fn count_antinodes_all(data: &[u8]) -> i32 {
    let data = parse_data(data);
    let height = data.len() as i32;
    let width = data[0].len() as i32;

    let antennas = get_antennas(&data);

    let antinodes = get_antinodes_all(&antennas, width, height);

    antinodes.len() as i32
}

mod test {
    use super::*;

    #[test]
    fn test_get_antinodes() {
        let mut antennas = vec![];
        antennas.push(Antenna { typ: b'0', row: 1, col: 1 });
        antennas.push(Antenna { typ: b'0', row: 2, col: 2 });

        let mut antinodes: Vec<_> = get_antinodes_all(&antennas, 6, 6).into_iter().collect();
        antinodes.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(antinodes.len(), 6);

        assert_eq!(antinodes[0], (0, 0));
        assert_eq!(antinodes[1], (1, 1));
        assert_eq!(antinodes[2], (2, 2));
        assert_eq!(antinodes[3], (3, 3));
        assert_eq!(antinodes[4], (4, 4));
        assert_eq!(antinodes[5], (5, 5));
    }

    #[test]
    fn test_count_antinodes() {
        let result = count_antinodes_all(EXAMPLE_DATA);
        assert_eq!(result, 34);
    }

    #[test]
    fn test_day_8_part_21() {
        let result = count_antinodes_all(include_bytes!("../data/day_8_input.txt"));
        assert_eq!(result, 766);
    }
}