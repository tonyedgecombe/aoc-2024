

pub fn parse_data(data: &[u8]) -> Vec<Vec<u8>> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.to_vec())
        .collect()
}

pub fn next_plot_position(map: &Vec<Vec<u8>>) -> Option<(usize, usize)> {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] != 0 {
                return Some((row, col));
            }
        }
    }

    None
}

pub fn remove_plot(map: &mut Vec<Vec<u8>>, removed: &mut Vec<Vec<u8>>, (row, col): (usize, usize), id: u8) {
    if id != map[row][col] {
        return;
    }

    removed[row][col] = id;
    map[row][col] = 0;

    if row > 0 {
        remove_plot(map, removed, (row - 1, col), id);
    }

    if row < map.len() - 1 {
        remove_plot(map, removed, (row + 1, col), id);
    }

    if col > 0 {
        remove_plot(map, removed, (row, col - 1), id);
    }

    if col < map[0].len() - 1 {
        remove_plot(map, removed, (row, col + 1), id);
    }
}

pub fn row_area(row: &Vec<u8>) -> usize {
    row.iter().filter(|&&c| c != 0).count()
}

pub fn area(map: &Vec<Vec<u8>>) -> usize {
    map.iter().map(|row| row_area(row)).sum()
}

pub const EXAMPLE_DATA: &[u8] = b"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        let map = parse_data(b"AAAA\nBBCD\nBBCC\nEEEC");

        assert_eq!(map.len(), 4);
        assert_eq!(map[0], b"AAAA");
        assert_eq!(map[3], b"EEEC");
    }

    #[test]
    fn test_next_plot_position() {
        let map = parse_data(b"AAAA\nBBCD\nBBCC\nEEEC");
        let Some(pos) = next_plot_position(&map) else {
            panic!("Unexpected next plot position");
        };

        assert_eq!(pos, (0, 0));
    }

    #[test]
    fn test_remove_plot() {
        let mut map = parse_data(b"AAAA\nBBCD\nBBCC\nEEEC");
        let mut plot_map = vec![vec![0; 4]; 4];

        remove_plot(&mut map, &mut plot_map, (0, 0), b'A');

        assert_eq!(map, parse_data(b"\0\0\0\0\nBBCD\nBBCC\nEEEC"));
        assert_eq!(plot_map, parse_data(b"AAAA\n\0\0\0\0\n\0\0\0\0\n\0\0\0\0"));
    }

    #[test]
    fn test_area() {
        let mut map = parse_data(b"AAAA\nBBCD\nBBCC\nEEEC");
        let mut plot_map = vec![vec![0; 4]; 4];

        remove_plot(&mut map, &mut plot_map, (0, 0), b'A');
        let area = area(&plot_map);

        assert_eq!(area, 4);
    }
}