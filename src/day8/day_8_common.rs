#[derive(Debug, PartialEq)]
pub struct Antenna {
    pub typ: u8,
    pub row: i32,
    pub col: i32,
}

pub fn parse_data(data: &[u8]) -> Vec<&[u8]> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| line.len() > 0)
        .collect()
}

pub fn get_antennas(data: &[&[u8]]) -> Vec<Antenna> {
    let mut result = vec![];

    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] == b'.' {
                continue
            }

            let antenna = Antenna { typ: data[row][col], row: row as i32, col: col as i32 };
            result.push(antenna);
        }
    }
    result
}

pub fn within_bounds(antinode_1: (i32, i32), width: i32, height: i32) -> bool {
    antinode_1.0 >= 0 && antinode_1.0 < height && antinode_1.1 >= 0 && antinode_1.1 < width
}


pub const EXAMPLE_DATA: &[u8] = b"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............

";

mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let lines = parse_data(EXAMPLE_DATA);

        assert_eq!(lines.len(), 12);
        assert_eq!(lines[0].len(), 12);
    }



    #[test]
    fn test_get_antennas() {
        let lines = parse_data(EXAMPLE_DATA);
        let antennas = get_antennas(&lines);

        assert_eq!(antennas.len(), 7);
        assert_eq!(antennas[0], Antenna {
            typ: b'0',
            row: 1,
            col: 8,
        });
    }
}