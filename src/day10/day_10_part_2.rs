use crate::day10::day_10_common::*;

fn count_trails_from_location(location: (usize, usize), map: &Vec<Vec<i64>>) -> usize {
    find_trail_ends(location, map).len()
}

pub fn count_trails(map: &Vec<Vec<i64>>) -> usize {
    find_trailheads(map)
        .into_iter()
        .map(|location| count_trails_from_location(location, map))
        .sum()
}

mod test {
    use super::*;

    #[test]
    fn day_10_part_1() {
        let map = parse_data(include_bytes!("../data/day_10_input.txt"));
        let result = count_trails(&map);

        assert_eq!(result, 1925);
    }

    #[test]
    fn test_count_trails() {
        let map = parse_data(&EXAMPLE_DATA);
        let result = count_trails(&map);
        assert_eq!(result, 81);
    }

    #[test]
    fn test_find_trail_ends_up() {
        let map = parse_data(b"09\n08");
        let location = (1, 1);
        let trail_ends = find_trail_ends(location, &map);

        assert_eq!(trail_ends, [(0, 1)]);
    }

    #[test]
    fn test_find_trail_ends_multiple() {
        let map = parse_data(b"89\n90");
        let location = (0, 0);
        let mut trail_ends = find_trail_ends(location, &map);
        trail_ends.sort_by(|l, r| l.0.cmp(&r.0));

        assert_eq!(trail_ends, [(0, 1), (1,0)]);
    }
}