use std::collections::HashMap;
use itertools::Itertools;

pub(crate) fn parse_data(data: &[u8]) -> Vec<&[u8]> {
    data
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .collect()
}

fn get_numeric_map() -> HashMap<u8, (i64, i64)> {
    let mut hash_map = HashMap::new();

    hash_map.insert(b'7', (0, 0));
    hash_map.insert(b'8', (0, 1));
    hash_map.insert(b'9', (0, 2));

    hash_map.insert(b'4', (1, 0));
    hash_map.insert(b'5', (1, 1));
    hash_map.insert(b'6', (1, 2));

    hash_map.insert(b'1', (2, 0));
    hash_map.insert(b'2', (2, 1));
    hash_map.insert(b'3', (2, 2));

    hash_map.insert(b'0', (3, 1));
    hash_map.insert(b'A', (3, 2));

    hash_map
}

fn get_key_map() -> HashMap<u8, (i64, i64)> {
    let mut hash_map = HashMap::new();

    hash_map.insert(b'^', (0, 1));
    hash_map.insert(b'A', (0, 2));

    hash_map.insert(b'<', (1, 0));
    hash_map.insert(b'v', (1, 1));
    hash_map.insert(b'>', (1, 2));

    hash_map
}

fn count_moves(depth: usize, maps: &[&HashMap<u8, (i64, i64)>], sequence: Vec<u8>) -> i64 {
    let mut cache: HashMap<(usize, Vec<u8>), i64> = HashMap::new();

    count_moves_with_cache(depth, maps, sequence, &mut cache)
}

fn count_moves_with_cache(depth: usize, maps: &[&HashMap<u8, (i64, i64)>], sequence: Vec<u8>, cache: &mut HashMap<(usize, Vec<u8>), i64>) -> i64 {
    if depth == 0 {
        return 1;
    }

    if let Some(count) = cache.get(&(depth, sequence.clone())) {
        return *count;
    }

    let map = &maps[0];
    let maps = &[maps[1], maps[1]];

    let mut result = 0;
    let mut position = *map.get(&b'A').unwrap();
    for step in sequence.clone() {
        let destination = *map.get(&step).unwrap();
        let sub_sequence = get_sequence(map, position, &destination);
        let length = sub_sequence.len();

        let permutations = sub_sequence
            .into_iter()
            .permutations(length)
            .unique()
            .filter(|permutation| valid(map, position, permutation));

        result += permutations.into_iter().map(|permutation| {
            let mut permutation: Vec<u8> = permutation.into_iter().collect();
            permutation.push(b'A');
            count_moves_with_cache(depth - 1, maps, permutation, cache)
        }).min().unwrap();

        position = destination;
    }

    cache.insert((depth, sequence), result);

    result
}

fn valid(map: &HashMap<u8, (i64, i64)>, start: (i64, i64), sequence: &Vec<u8>) -> bool {
    let mut pos = start;

    for step in sequence {
        pos = match step {
            b'<' => (pos.0, pos.1 - 1),
            b'>' => (pos.0, pos.1 + 1),
            b'^' => (pos.0 - 1, pos.1),
            b'v' => (pos.0 + 1, pos.1),
            _ => panic!()
        };

        if !map.iter().any(|tuple| pos == *tuple.1) {
            return false;
        }
    }

    true
}

fn get_sequence(map: &HashMap<u8, (i64, i64)>, current: (i64, i64), destination: &(i64, i64)) -> Vec<u8> {
    let mut steps = Vec::new();

    // Vertical steps
    for _ in 0..i64::abs(destination.0 - current.0) {
        if destination.0 < current.0 {
            steps.push(b'^');
        } else if destination.0 > current.0 {
            steps.push(b'v');
        }
    }

    // Horizontal steps
    for _ in 0..i64::abs(destination.1 - current.1) {
        if destination.1 < current.1 {
            steps.push(b'<');
        } else if destination.1 > current.1 {
            steps.push(b'>');
        }
    }

    if valid(map, current, &steps) {
        return steps;
    } else {
        steps.clear();
    }

    // Horizontal steps
    for _ in 0..i64::abs(destination.1 - current.1) {
        if destination.1 < current.1 {
            steps.push(b'<');
        } else if destination.1 > current.1 {
            steps.push(b'>');
        }
    }

    // Vertical steps
    for _ in 0..i64::abs(destination.0 - current.0) {
        if destination.0 < current.0 {
            steps.push(b'^');
        } else if destination.0 > current.0 {
            steps.push(b'v');
        }
    }

    if !valid(map, current, &steps) {
        panic!()
    }

    steps
}


fn get_numeric_part_of_code(code: &[u8]) -> i64 {
    let mut result = 0;

    for c in code {
        if c >= &b'0' && c <= &b'9' {
            result = result * 10 + (c - b'0') as i64;
        }
    }

    result
}

fn play_sequence(map: &HashMap<u8, (i64, i64)>, sequence: &[u8]) -> Vec<u8> {
    let mut pos = *map.get(&b'A').unwrap();
    let inverted_map: HashMap<(i64, i64), u8> = map.into_iter().map(|(k, v)|(*v, *k)).collect();
    let mut result = Vec::new();

    for c in sequence {
        match c {
            b'<' => pos = (pos.0, pos.1 - 1),
            b'>' => pos = (pos.0, pos.1 + 1),
            b'^' => pos = (pos.0 - 1, pos.1),
            b'v' => pos = (pos.0 + 1, pos.1),
            b'A' => {
                let key = *inverted_map.get(&pos).unwrap();
                result.push(key);
            }
            _ => panic!()
        }
    }

    result
}

pub fn calculate_complexity(codes: &Vec<&[u8]>, depth: usize) -> i64 {
    let numeric_map = get_numeric_map();
    let key_map = get_key_map();

    codes.iter().map(|code| {
        let num_part = get_numeric_part_of_code(code);
        let moves = count_moves(depth, &[&numeric_map, &key_map], code.to_vec());
        num_part * moves
    }).sum::<i64>()
}

pub const EXAMPLE_DATA: &[u8] = b"
029A
980A
179A
456A
379A";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let lines = parse_data(EXAMPLE_DATA);
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[2], b"179A");
    }

    #[test]
    fn test_valid() {
        let map = get_numeric_map();

        assert!(!valid(&map, (3, 2), &b"<<^".to_vec()));
        assert!(valid(&map, (3, 2), &b"^<<".to_vec()));
    }

    #[test]
    fn test_get_sequence_up_and_left() {
        let map = get_numeric_map();
        let start = map.get(&b'A').unwrap();
        let destination = map.get(&b'1').unwrap();
        let sequence = get_sequence(&map, *start, destination);

        assert_eq!(sequence, b"^<<");
    }

    #[test]
    fn test_count_depth_2() {
        let numeric_map = get_numeric_map();
        let key_map = get_key_map();

        let count = count_moves(2, &[&numeric_map, &key_map], b"029A".to_vec());
        assert_eq!(count, 12);
    }

    #[test]
    fn test_count_depth_3() {
        let numeric_map = get_numeric_map();
        let key_map = get_key_map();

        let count = count_moves(3, &[&numeric_map, &key_map], b"029A".to_vec());
        assert_eq!(count, 28);
    }

    #[test]
    fn test_count_depth_4() {
        let numeric_map = get_numeric_map();
        let key_map = get_key_map();

        let count = count_moves(4, &[&numeric_map, &key_map], b"029A".to_vec());
        assert_eq!(count, 68);
    }

    #[test]
    fn test_count_depth_4_379a() {
        let numeric_map = get_numeric_map();
        let key_map = get_key_map();

        let count = count_moves(4, &[&numeric_map, &key_map], b"379A".to_vec());
        assert_eq!(count, 64);
    }

    #[test]
    fn test_count_depth_4_179a() {
        let numeric_map = get_numeric_map();
        let key_map = get_key_map();

        let count = count_moves(4, &[&numeric_map, &key_map], b"179A".to_vec());
        assert_eq!(count, 68);
    }

    #[test]
    fn test_play_sequence_379a() {
        let numeric_map = get_numeric_map();
        let key_map = get_key_map();

        let sequence = b"<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A";
        let sequence = play_sequence(&key_map, sequence);

        let sequence = play_sequence(&key_map, &sequence);

        let sequence = play_sequence(&numeric_map, &sequence);
        assert_eq!(sequence, b"379A");
    }

    #[test]
    fn test_get_numeric_part_of_code() {
        assert_eq!(get_numeric_part_of_code(b"029A"), 29);
        assert_eq!(get_numeric_part_of_code(b"980A"), 980);
    }

    #[test]
    fn test_calculate_complexity() {
        let codes = parse_data(EXAMPLE_DATA);
        let result = calculate_complexity(&codes, 4);
        assert_eq!(result, 126384);
    }
}