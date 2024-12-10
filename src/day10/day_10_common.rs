
pub fn parse_data(data: &[u8]) -> Vec<Vec<i64>> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.iter().map(|&c| (c - b'0') as i64).collect())
        .collect()
}

pub fn find_trailheads(data: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if data[row][col] == 0 {
                result.push((row, col));
            }
        }
    }

    result
}

pub fn find_trail_ends(location: (usize, usize), map: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    let map_height = map.len();
    let map_width = map[0].len();

    let height = map[location.0][location.1];

    let mut result = Vec::new();

    if height == 9 {
        result.push(location);
        return result;
    }

    // Up
    if location.0 > 0 && map[location.0 - 1][location.1] == height + 1 {
        let up = find_trail_ends((location.0 - 1, location.1), map);
        result.extend(&up);
    }

    // Down
    if location.0 < map_height - 1 && map[location.0 + 1][location.1] == height + 1 {
        let down = find_trail_ends((location.0 + 1, location.1), map);
        result.extend(&down);
    }

    // Left
    if location.1 > 0 && map[location.0][location.1 -1] == height + 1 {
        let left = find_trail_ends((location.0, location.1 - 1), map);
        result.extend(&left);
    }

    // Right
    if location.1 < map_width - 1 && map[location.0][location.1 + 1] == height + 1 {
        let right = find_trail_ends((location.0, location.1 + 1), map);
        result.extend(&right);
    }

    result
}

pub const EXAMPLE_DATA: &[u8] = b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";


mod test {
    use super::*;


    #[test]
    fn test_parse_data() {
        let result = parse_data(EXAMPLE_DATA);
        assert_eq!(result.len(), 8);
        assert_eq!(result[1], [7, 8, 1 ,2, 1, 8, 7, 4]);
    }

    #[test]
    fn test_find_trailheads() {
        let map = parse_data(EXAMPLE_DATA);
        let trailheads = find_trailheads(&map);

        assert_eq!(trailheads.len(), 9);
    }
}