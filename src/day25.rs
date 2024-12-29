enum Item {
    Lock {heights: [usize; 5]},
    Key {heights: [usize; 5]},
}

fn parse_item(data: &[u8]) -> Item {
    let lines: Vec<_> = data.split(|&c| c == b'\n').filter(|line| !line.is_empty()).collect();
    let mut heights = [0; 5];

    if lines[0] == b"#####" {
        for col in 0..heights.len() {
            for row in 0..lines.len() {
                if lines[row][col] == b'.' {
                    heights[col] = row - 1;
                    break;
                }
            }
        }

        Item::Lock { heights }
    } else {
        for col in 0..heights.len() {
            for row in (0..lines.len()).rev() {
                if lines[row][col] == b'.' {
                    heights[col] = lines.len() - row - 2;
                    break;
                }
            }
        }

        Item::Key { heights }
    }
}

fn parse_items(data: &[u8]) -> (Vec<Item>, Vec<Item>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut start_pos = 0;
    for i in 0..data.len() - 1 {
        if data[i] == b'\n' && data[i+1] == b'\n' {
            let item = parse_item(&data[start_pos..i]);
            match item {
                Item::Lock { .. } => locks.push(item),
                Item::Key { .. } => keys.push(item),
            }

            start_pos = i + 2;
        }
    }

    let item = parse_item(&data[start_pos..]);
    match item {
        Item::Lock { .. } => locks.push(item),
        Item::Key { .. } => keys.push(item),
    }

    (locks, keys)
}

fn count_matches(data: &[u8]) -> usize {
    let (locks, keys) = parse_items(data);

    let mut count = 0;
    for lock in &locks {
        let Item::Lock { heights: lock_heights } = lock else {
            panic!();
        };

        for key in &keys {
            let Item::Key { heights: key_heights } = key else {
                panic!();
            };

            if (0..5).all(|i| lock_heights[i] + key_heights[i] <= 5) {
                count += 1;
            }
        }
    }

    count
}

fn day_25() -> usize {
    let data = include_bytes!("data/day_25_input.txt");
    count_matches(data)
}

const EXAMPLE_DATA: &[u8] = b"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_item_lock() {
        let data = b"#####\n.####\n.####\n.####\n.#.#.\n.#...\n.....";
        let Item::Lock { heights } = parse_item(data) else {
            panic!();
        };

        assert_eq!(heights, [0, 5, 3, 4, 3]);
    }

    #[test]
    fn test_parse_item_key() {
        let data = b".....\n#....\n#....\n#...#\n#.#.#\n#.###\n#####";
        let Item::Key { heights } = parse_item(data) else {
            panic!();
        };

        assert_eq!(heights, [5, 0, 2, 1, 3]);
    }

    #[test]
    fn test_parse_data() {
        let (locks, keys) = parse_items(EXAMPLE_DATA);
        assert_eq!(locks.len(), 2);
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn test_count_matches_example_data() {
        let result = count_matches(EXAMPLE_DATA);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_day_25() {
        let result = day_25();
        assert_eq!(result, 3127);
    }
}