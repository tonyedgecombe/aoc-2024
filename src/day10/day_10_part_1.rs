use std::collections::HashSet;
use crate::day10::day_10_common::*;

pub fn find_distinct_trail_ends(location: (usize, usize), map: &Vec<Vec<i64>>) -> HashSet<(usize, usize)> {
    let trail_ends = find_trail_ends(location, map);
    HashSet::from_iter(trail_ends.into_iter())
}

fn count_distinct_trails_from_location(location: (usize, usize), map: &Vec<Vec<i64>>) -> usize {
    find_distinct_trail_ends(location, map).len()
}

pub fn count_distinct_trails(map: &Vec<Vec<i64>>) -> usize {
    find_trailheads(map)
        .into_iter()
        .map(|location| count_distinct_trails_from_location(location, map))
        .sum()
}

mod test {
    use super::*;

    #[test]
    fn test_count_trails() {
        let map = parse_data(&EXAMPLE_DATA);
        let result = count_distinct_trails(&map);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_count_trails_from_location() {
        let map = parse_data(&EXAMPLE_DATA.to_vec());
        let location = (0, 2);
        let result = count_distinct_trails_from_location(location, &map);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_find_trail_ends_termination() {
        let map = parse_data(&EXAMPLE_DATA.to_vec());
        let location = (0, 1);
        let trail_ends = find_distinct_trail_ends(location, &map);
        assert_eq!(trail_ends.iter().collect::<Vec<_>>(), [&(0, 1)]);
    }

    #[test]
    fn test_find_trail_ends_up() {
        let map = parse_data(b"09\n08");
        let location = (1, 1);
        let trail_ends = find_distinct_trail_ends(location, &map);

        assert_eq!(trail_ends.iter().collect::<Vec<_>>(), [&(0, 1)]);
    }

    #[test]
    fn test_find_trail_ends_down() {
        let map = parse_data(b"08\n09");
        let location = (0, 1);
        let trail_ends = find_distinct_trail_ends(location, &map);

        assert_eq!(trail_ends.iter().collect::<Vec<_>>(), [&(1,1)]);
    }

    #[test]
    fn test_find_trail_ends_left()  {
        let map = parse_data(b"98\n00");
        let location = (0, 1);
        let trail_ends = find_distinct_trail_ends(location, &map);

        assert_eq!(trail_ends.iter().collect::<Vec<_>>(), [&(0,0)]);
    }

    #[test]
    fn test_find_trail_ends_right() {
        let map = parse_data(b"89\n00");
        let location = (0, 0);
        let trail_ends = find_distinct_trail_ends(location, &map);

        assert_eq!(trail_ends.iter().collect::<Vec<_>>(), [&(0,1)]);
    }

    #[test]
    fn test_find_trail() {
        let map = parse_data(b"67\n98");
        let location = (0, 0);
        let trail_ends = find_distinct_trail_ends(location, &map);

        assert_eq!(trail_ends.iter().collect::<Vec<_>>(), [&(1,0)]);
    }

    #[test]
    fn test_find_trail_ends_no_trail() {
        let map = parse_data(b"67\n58");
        let location = (0, 0);
        let trail_ends = find_distinct_trail_ends(location, &map);

        assert!(trail_ends.is_empty());
    }

    #[test]
    fn day_10_part_1() {
        let data = parse_data(include_bytes!("../data/day_10_input.txt"));
        let result = count_distinct_trails(&data);
        assert_eq!(result, 778);
    }
}