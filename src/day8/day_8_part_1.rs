use std::collections::HashSet;
use crate::day8::day_8_common::*;

fn get_antinodes(antennas: &Vec<Antenna>, width: i32, height: i32) -> HashSet<(i32, i32)> {
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

            let antinode = (first.row + d_row, first.col + d_col);
            if within_bounds(antinode, width, height) {
                result.insert(antinode);
            }

            let antinode = (second.row - d_row, second.col - d_col);
            if within_bounds(antinode, width, height) {
                result.insert(antinode);
            }
        }
    }

    result
}

fn count_antinodes(data: &[u8]) -> i32 {
    let data = parse_data(data);
    let height = data.len() as i32;
    let width = data[0].len() as i32;

    let antennas = get_antennas(&data);

    let antinodes = get_antinodes(&antennas, width, height);

    antinodes.len() as i32
}



mod test {
    use crate::day8::day_8_common::{Antenna, EXAMPLE_DATA};
    use super::*;

    #[test]
    fn test_get_antinodes() {
        let mut antennas = vec![];
        antennas.push(Antenna { typ: b'0', row: 2, col: 2 });
        antennas.push(Antenna { typ: b'0', row: 4, col: 4 });

        let mut antinodes: Vec<_> = get_antinodes(&antennas, 8, 8).into_iter().collect();
        antinodes.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(antinodes.len(), 2);

        assert_eq!(antinodes[0], (0, 0));
        assert_eq!(antinodes[1], (6, 6));
    }

    #[test]
    fn test_get_antinodes_out_of_bounds() {
        let mut antennas = vec![];
        antennas.push(Antenna { typ: b'0', row: 2, col: 2 });
        antennas.push(Antenna { typ: b'0', row: 4, col: 4 });

        let antinodes = get_antinodes(&antennas, 6, 6);
        assert_eq!(antinodes.len(), 1);
        let mut it = antinodes.iter();
        assert_eq!(it.next().unwrap(), &(0, 0));
    }

    #[test]
    fn test_count_antinodes() {
        let result = count_antinodes(EXAMPLE_DATA);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_day_8_part_1() {
        let result = count_antinodes(include_bytes!("../data/day_8_input.txt"));
        assert_eq!(result, 228);
    }
}